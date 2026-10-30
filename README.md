<p align="center"><img src="https://raw.githubusercontent.com/glasspadotfun/glasspadotfun/main/brand/mark.png" width="160" alt=""></p>

# glasspad-program

the glasspad curve program (anchor 0.31). a fork of a public reconstruction of the other pad's curve program, with every change as its own commit so the diff page can list them by name.

## what changes, and why

| change | why |
|---|---|
| every admin instruction deleted | the rules can't move after launch |
| curve constants and fee rates are `const`s | the rules are in the source, not in an account someone writes |
| platform fee split hardcoded 50 / 50: team wallet / burn pda | no vault, nothing trades with fees |
| side modes removed (mayhem, cashback, incentives, holder rewards, non-sol quotes, multi-hop) | each one was an admin lever or a cpi into a program we don't deploy |
| `migrate` goes to raydium cpmm and burns the lp in the same instruction | permissionless graduation, nothing to pull |
| `burn_crank`, new and permissionless | swaps the burn pda's sol for the platform coin and burns it |

## verify it yourself

filled in at deploy: program id, verified build hash, `solana-verify` command, idl, upgrade authority (`none`), every pda.

## status

pre-launch. nothing is deployed yet.

mit licence. unaudited.
