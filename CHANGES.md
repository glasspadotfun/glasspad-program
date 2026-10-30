# changes

each line is one commit, in this order. a line is done when the commit exists and the build passes.

1. [x] delete every admin instruction (entry points; the dead handler modules go in change 4) (`set_params`, `update_creator_fee_config`, `update_global_authority`, `admin_set_idl_authority`, `admin_cto`, `set_reserved_fee_recipients`, `set_buyback_basis_points`, `set_buyback_fee_recipients`, `update_buyback_config`, `set_max_curve_depth`, `set_virtual_quote_reserves`, `set_mayhem_virtual_params`, `toggle_*`, `admin_update_token_incentives`, every `*_quote_control_*`, `update_holder_reward_config`, `set_creator`, `set_metaplex_creator`)
2. [x] curve constants and fee rates become `const`s: initial virtual sol / token reserves, real token reserves, total supply, platform fee 100 bps, creator fee 40 bps, migration fee 1 sol. `initialize` writes them once and keeps no authority
3. [x] fee split hardcoded (done by change 2: initialize pins both recipients and 50%; nothing can rewrite them): `buyback_basis_points = 5000`, `fee_recipient` = team wallet const, `buyback_fee_recipient` = burn pda. remove `whitelist_pda` (zero-fee bypass)
4. [x] side features removed (entry points + the fee program and volume accumulators out of buy/sell; dead modules still compile and go in change 7): mayhem, cashback, token incentives + volume accumulators, holder rewards, quote-control mints, multi-hop, the fee-program cpi (fees from the consts)
5. [ ] `migrate` → raydium cpmm (`CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C`), lp burned in the same instruction, permissionless
6. [ ] `burn_crank`: permissionless swap of the burn pda's sol for the platform coin on its cpmm pool, min-out from reserves, then burn, with an event
7. [ ] program id, pda seeds, event names, error strings renamed; crate renamed `glasspad-curve`

then: devnet deploy + canary, identity build + score, verified build, idl on chain, erase idl authority, burn the upgrade key (mainnet, as the launch event).
