// glasspad canary: one real coin through the whole program on devnet (or mainnet).
// initialize → create → buy → sell → buy to completion → migrate (raydium cpmm, lp burned)
// → seal_platform_coin (the canary coin plays the platform coin) → burn_crank.
// every signature lands in RUN.md next to this file.
import fs from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import anchorPkg from '@coral-xyz/anchor';
import { Connection, Keypair, PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, LAMPORTS_PER_SOL, ComputeBudgetProgram } from '@solana/web3.js';
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID, NATIVE_MINT, getAssociatedTokenAddressSync, createAssociatedTokenAccountIdempotentInstruction, getAccount } from '@solana/spl-token';

const { AnchorProvider, Program, Wallet, BN } = anchorPkg;
const cluster = process.argv[2] || 'devnet';
const here = path.dirname(new URL(import.meta.url).pathname);
const idl = JSON.parse(fs.readFileSync(process.env.IDL_PATH || path.join(here, '..', 'program', 'out', 'glasspad_curve.json'), 'utf8'));
const deployer = Keypair.fromSecretKey(Uint8Array.from(JSON.parse(fs.readFileSync(path.join(os.homedir(), '.config/glasspad/deployer.json'), 'utf8'))));

const RPC = process.env.RPC_URL || (cluster === 'devnet' ? 'https://api.devnet.solana.com' : 'https://api.mainnet-beta.solana.com');
const CPMM = cluster === 'devnet'
  ? { program: new PublicKey('DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb'), config: new PublicKey('5MxLgy9oPdTC3YgkiePHqr3EoCRD9uLVYRQS2ANAs7wy'), feeReceiver: new PublicKey('3oE58BKVt8KuYkGxx8zBojugnymWmBiyafWgMrnb6eYy') }
  : { program: new PublicKey('CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C'), config: new PublicKey('D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2'), feeReceiver: new PublicKey('DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8') };
const TEAM = new PublicKey(cluster === 'devnet' ? deployer.publicKey : '6r8QzyMGZ12rsDFYDakrM7j5i52VNY28KKZTTQRWikGs');
const MPL = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s');

const connection = new Connection(RPC, 'confirmed');
const provider = new AnchorProvider(connection, new Wallet(deployer), { commitment: 'confirmed' });
const program = new Program(idl, provider);
const PID = program.programId;

const pda = (seeds, pid = PID) => PublicKey.findProgramAddressSync(seeds, pid)[0];
const global = pda([Buffer.from('global')]);
const burnPda = pda([Buffer.from('burn')]);
const eventAuthority = pda([Buffer.from('__event_authority')]);
const log = [];
const note = (step, sig, extra = '') => { const line = `- ${step}: ${sig ? `[\`${sig}\`](https://solscan.io/tx/${sig}${cluster === 'devnet' ? '?cluster=devnet' : ''})` : ''} ${extra}`.trim(); console.log(line); log.push(line); };
const send = async (step, builder, signers = []) => {
  const tx = await builder.preInstructions([ComputeBudgetProgram.setComputeUnitLimit({ units: 600_000 })]).signers(signers).rpc({ commitment: 'confirmed' });
  note(step, tx); return tx;
};
const cu = (x) => Number(x) / LAMPORTS_PER_SOL;

async function main() {
  note(`canary on ${cluster}`, null, `program ${PID}, deployer ${deployer.publicKey}, ${new Date().toISOString()}`);
  const bal = await connection.getBalance(deployer.publicKey);
  note('deployer balance', null, `${cu(bal)} sol`);

  // 1. initialize (once)
  if (!(await connection.getAccountInfo(global))) {
    await send('initialize', program.methods.initialize().accounts({ global, user: deployer.publicKey, systemProgram: SystemProgram.programId }));
  } else note('initialize', null, 'global already exists, skipped');
  const g = await program.account.global.fetch(global);
  note('global', null, `fee ${g.feeBasisPoints} bps, creator ${g.creatorFeeBasisPoints} bps, buyback ${g.buybackBasisPoints} bps, migration fee ${cu(g.poolMigrationFee)} sol, team ${g.feeRecipient}, burn ${g.buybackFeeRecipients[0]}`);

  // 2. create
  const mint = Keypair.generate();
  const bondingCurve = pda([Buffer.from('bonding-curve'), mint.publicKey.toBuffer()]);
  const bondingCurveV2 = pda([Buffer.from('bonding-curve-v2'), mint.publicKey.toBuffer()]);
  const associatedBondingCurve = getAssociatedTokenAddressSync(mint.publicKey, bondingCurve, true);
  const metadata = PublicKey.findProgramAddressSync([Buffer.from('metadata'), MPL.toBuffer(), mint.publicKey.toBuffer()], MPL)[0];
  await send('create', program.methods.create('glasspad canary', 'CANARY', 'https://glasspad.fun/canary.json', deployer.publicKey).accounts({
    mint: mint.publicKey, mintAuthority: pda([Buffer.from('mint-authority')]), bondingCurve, associatedBondingCurve, global,
    mplTokenMetadata: MPL, metadata, user: deployer.publicKey, systemProgram: SystemProgram.programId, tokenProgram: TOKEN_PROGRAM_ID,
    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID, rent: SYSVAR_RENT_PUBKEY, eventAuthority, program: PID,
  }), [mint]);
  note('mint', null, `${mint.publicKey}`);

  // 3. trade
  const associatedUser = getAssociatedTokenAddressSync(mint.publicKey, deployer.publicKey);
  const creatorVault = pda([Buffer.from('creator-vault'), deployer.publicKey.toBuffer()]);
  const tradeAccounts = { global, feeRecipient: TEAM, mint: mint.publicKey, bondingCurve, associatedBondingCurve, associatedUser, user: deployer.publicKey, systemProgram: SystemProgram.programId, tokenProgram: TOKEN_PROGRAM_ID, creatorVault, eventAuthority, program: PID };
  const remaining = [{ pubkey: bondingCurveV2, isSigner: false, isWritable: false }, { pubkey: burnPda, isSigner: false, isWritable: true }];
  const ataIx = createAssociatedTokenAccountIdempotentInstruction(deployer.publicKey, associatedUser, deployer.publicKey, mint.publicKey);
  const curve = async () => program.account.bondingCurve.fetch(bondingCurve);

  await send('buy 0.2 sol', program.methods.buyExactSolIn(new BN(0.2 * LAMPORTS_PER_SOL), new BN(0), [false], [true]).accounts(tradeAccounts).remainingAccounts(remaining).preInstructions([ataIx]));
  let c = await curve(); note('curve after buy', null, `real sol ${cu(c.realQuoteReserves)}, real tokens ${c.realTokenReserves.toString()}`);
  const held = (await getAccount(connection, associatedUser)).amount;
  await send('sell half', program.methods.sell(new BN((held / 2n).toString()), new BN(0)).accounts(tradeAccounts).remainingAccounts(remaining));
  c = await curve(); note('curve after sell', null, `real sol ${cu(c.realQuoteReserves)}`);
  for (let i = 0; i < 6 && !(await curve()).complete; i++) {
    await send(`buy to complete #${i + 1}`, program.methods.buyExactSolIn(new BN(0.5 * LAMPORTS_PER_SOL), new BN(0), [false], [true]).accounts(tradeAccounts).remainingAccounts(remaining));
  }
  c = await curve(); note('curve complete', null, `${c.complete}, real sol ${cu(c.realQuoteReserves)}, team ${cu(await connection.getBalance(TEAM))} sol, burn pda ${cu(await connection.getBalance(burnPda))} sol`);
  if (!c.complete) throw new Error('curve did not complete');

  // 4. migrate to raydium cpmm, lp burned
  const [t0, t1] = [mint.publicKey, NATIVE_MINT].sort((a, b) => a.toBuffer().compare(b.toBuffer()));
  const poolAuthority = pda([Buffer.from('pool-authority'), mint.publicKey.toBuffer()]);
  const cpmmAuthority = pda([Buffer.from('vault_and_lp_mint_auth_seed')], CPMM.program);
  const poolState = pda([Buffer.from('pool'), CPMM.config.toBuffer(), t0.toBuffer(), t1.toBuffer()], CPMM.program);
  const lpMint = pda([Buffer.from('pool_lp_mint'), poolState.toBuffer()], CPMM.program);
  const vault = (m) => pda([Buffer.from('pool_vault'), poolState.toBuffer(), m.toBuffer()], CPMM.program);
  const observation = pda([Buffer.from('observation'), poolState.toBuffer()], CPMM.program);
  await send('migrate', program.methods.migrate().accounts({
    global, mint: mint.publicKey, bondingCurve, associatedBondingCurve, user: deployer.publicKey, poolAuthority,
    poolAuthorityMintAccount: getAssociatedTokenAddressSync(mint.publicKey, poolAuthority, true),
    poolAuthorityWsolAccount: getAssociatedTokenAddressSync(NATIVE_MINT, poolAuthority, true),
    poolAuthorityLpAccount: getAssociatedTokenAddressSync(lpMint, poolAuthority, true),
    wsolMint: NATIVE_MINT, team: TEAM, burnPda, cpmmProgram: CPMM.program, ammConfig: CPMM.config, cpmmAuthority,
    token0Mint: t0, token1Mint: t1, poolState, lpMint, token0Vault: vault(t0), token1Vault: vault(t1), createPoolFee: CPMM.feeReceiver, observationState: observation,
    tokenProgram: TOKEN_PROGRAM_ID, associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID, systemProgram: SystemProgram.programId, rent: SYSVAR_RENT_PUBKEY, eventAuthority, program: PID,
  }));
  note('pool', null, `${poolState} (lp mint ${lpMint}), team ${cu(await connection.getBalance(TEAM))} sol, burn pda ${cu(await connection.getBalance(burnPda))} sol`);

  // 5. seal + burn crank (the canary coin plays the platform coin on devnet)
  if (cluster === 'devnet') {
    await send('seal_platform_coin', program.methods.sealPlatformCoin().accounts({ global, team: deployer.publicKey, platformCoin: mint.publicKey }));
    const before = await connection.getBalance(burnPda);
    await send('burn_crank', program.methods.burnCrank().accounts({
      global, user: deployer.publicKey, burnPda, platformCoin: mint.publicKey, wsolMint: NATIVE_MINT,
      burnWsolAccount: getAssociatedTokenAddressSync(NATIVE_MINT, burnPda, true), burnCoinAccount: getAssociatedTokenAddressSync(mint.publicKey, burnPda, true),
      cpmmProgram: CPMM.program, ammConfig: CPMM.config, cpmmAuthority, token0Mint: t0, token1Mint: t1, poolState, token0Vault: vault(t0), token1Vault: vault(t1), observationState: observation,
      tokenProgram: TOKEN_PROGRAM_ID, associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID, systemProgram: SystemProgram.programId, eventAuthority, program: PID,
    }));
    note('burn', null, `burn pda ${cu(before)} → ${cu(await connection.getBalance(burnPda))} sol`);
  }
  // 6. creator fee claim
  await send('collect_creator_fee', program.methods.collectCreatorFee().accounts({ creator: deployer.publicKey, creatorVault, systemProgram: SystemProgram.programId, eventAuthority, program: PID }));
  note('done', null, `deployer ${cu(await connection.getBalance(deployer.publicKey))} sol`);
}

main().catch((e) => { note('FAILED', null, String(e?.message || e)); if (e?.logs) console.log(e.logs.join('\n')); process.exitCode = 1; }).finally(() => {
  fs.appendFileSync(path.join(here, 'RUN.md'), `\n## ${cluster} ${new Date().toISOString()}\n${log.join('\n')}\n`);
});
