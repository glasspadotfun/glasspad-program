
## devnet 2026-10-30T12:10:47.914Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:10:45.809Z
- deployer balance:  2.4971408 sol
- initialize: [`2T4HAVy8JGw4Z1f2tWGJZP4NZm8aZ1LSXW82iqNi8psvKfuedRUmdJYMQoekkk24NQVgrr75JW4eeRvdhkCfnT6o`](https://solscan.io/tx/2T4HAVy8JGw4Z1f2tWGJZP4NZm8aZ1LSXW82iqNi8psvKfuedRUmdJYMQoekkk24NQVgrr75JW4eeRvdhkCfnT6o?cluster=devnet)
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`3mgS8ZT7jQQE2DwRZehQNFWLUNYerXLjMyYU5QbrYCnWcanU4NtHL17bC7maMxV9RVC1J8kRF4nrMXyKsP3PueMK`](https://solscan.io/tx/3mgS8ZT7jQQE2DwRZehQNFWLUNYerXLjMyYU5QbrYCnWcanU4NtHL17bC7maMxV9RVC1J8kRF4nrMXyKsP3PueMK?cluster=devnet)
- mint:  8WSqmPeDYEMqfGuDqV2i2FQ73MxXAkL5Cr65cLeeKRVj
- FAILED:  AnchorError thrown in src/buy.rs:57. Error Code: BuyZeroAmount. Error Number: 6020. Error Message: Buy zero amount.
Program log: Left: 0
Program log: Right: 0

## devnet 2026-10-30T12:16:50.567Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:16:45.397Z
- deployer balance:  2.47316596 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`3N9yPwFk9n3D1o7mNX7SCPyq1AsCoPCeXVu1pGkiExT37n6HQfvdcuGgW1AsGSZ3GZcZQ5xuvnBFz3EzapR9APWz`](https://solscan.io/tx/3N9yPwFk9n3D1o7mNX7SCPyq1AsCoPCeXVu1pGkiExT37n6HQfvdcuGgW1AsGSZ3GZcZQ5xuvnBFz3EzapR9APWz?cluster=devnet)
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- buy 0.2 sol: [`2DPsTKsDXJ3ej5FxCaUcEXGo4PCM4eBnSLtb6dM3R3nZtJqaGsEEtQxoJzXUmVfvTXZXnnXMxDov58SVFvxZ6des`](https://solscan.io/tx/2DPsTKsDXJ3ej5FxCaUcEXGo4PCM4eBnSLtb6dM3R3nZtJqaGsEEtQxoJzXUmVfvTXZXnnXMxDov58SVFvxZ6des?cluster=devnet)
- curve after buy:  real sol 0.197238658, real tokens 367475250231199
- sell half: [`GBjF8fon2AWxLQTCypTdVDTqrGWGvLECDFxJTRFsBifJQiHtmTLx4mUjjMzFjwzk25FhNe7pnNMLCMZpR4g1t9x`](https://solscan.io/tx/GBjF8fon2AWxLQTCypTdVDTqrGWGvLECDFxJTRFsBifJQiHtmTLx4mUjjMzFjwzk25FhNe7pnNMLCMZpR4g1t9x?cluster=devnet)
- curve after sell:  real sol 0.074220684
- buy to complete #1: [`4NpKRC1t3e13XmDcVopijeRGM1cpTjzUVeXheCmjWXBhrJKH8o26LWJpwqL3ckqSfLLLswLzACZFcuV7EqHMsWBa`](https://solscan.io/tx/4NpKRC1t3e13XmDcVopijeRGM1cpTjzUVeXheCmjWXBhrJKH8o26LWJpwqL3ckqSfLLLswLzACZFcuV7EqHMsWBa?cluster=devnet)
- buy to complete #2: [`as3DRVSAeSNC3Tf2QfPyurwFA1avKYMPxCRBDs9iLnAQMCrnMXH7YyZZcGPktjpCJ5zGNTYBS3fPXw5CNTbkpj9`](https://solscan.io/tx/as3DRVSAeSNC3Tf2QfPyurwFA1avKYMPxCRBDs9iLnAQMCrnMXH7YyZZcGPktjpCJ5zGNTYBS3fPXw5CNTbkpj9?cluster=devnet)
- curve complete:  true, real sol 0.850053598, team 1.593296315 sol, burn pda 0.005480447 sol
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: sum of account balances before and after instruction do not match. 
Logs: 
[
  "Program ComputeBudget111111111111111111111111111111 invoke [1]",
  "Program ComputeBudget111111111111111111111111111111 success",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH invoke [1]",
  "Program log: Instruction: Migrate",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 51849 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: sum of account balances before and after instruction do not match"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T12:37:04.505Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:37:04.139Z
- deployer balance:  6.537842115 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- curve after sell:  real sol 0.850053598
- curve complete:  true, real sol 0.850053598, team 6.537842115 sol, burn pda 0.005480447 sol
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: invalid program argument. 
Logs: 
[
  "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 76 of 509519 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program log: Error: InvalidArgument",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 184 of 507543 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA failed: invalid program argument",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 92491 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: invalid program argument"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T12:37:31.800Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:37:31.375Z
- deployer balance:  6.537842115 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- curve after sell:  real sol 0.850053598
- curve complete:  true, real sol 0.850053598, team 6.537842115 sol, burn pda 0.005480447 sol
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: invalid program argument. 
Logs: 
[
  "Program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL success",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 76 of 509519 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program log: Error: InvalidArgument",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 184 of 507543 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA failed: invalid program argument",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 92491 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: invalid program argument"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T12:42:31.535Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:42:29.191Z
- deployer balance:  6.534407115 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- curve after sell:  real sol 0.850053598
- curve complete:  true, real sol 0.850053598, team 6.534407115 sol, burn pda 0.005480447 sol
- migrate: [`2fGEoHHyhpKaMoAVn3e7KgnYG6ALe3drWqwiwqco4iyv6PrkgSasd799bLUBkhiJg52ygYHbjDKAg8Chf2hmHy8L`](https://solscan.io/tx/2fGEoHHyhpKaMoAVn3e7KgnYG6ALe3drWqwiwqco4iyv6PrkgSasd799bLUBkhiJg52ygYHbjDKAg8Chf2hmHy8L?cluster=devnet)
- pool:  C5S2LJWeDU2aeqvL1EYkhmpatApxNvrpkUhqVfsXQk8b (lp mint AfXe8e4xSrRUKk2Fttdpiy7JSvNWLacVHGhGhnLJBk6A), team 6.544761555 sol, burn pda 0.015839887 sol
- seal_platform_coin: [`5YoY4XZdgNvvQ8hMtYGHrzNdkS7MyabAg3FryvNReP6ufo2Hg7d9CfZpr3RyxAc37Fnjm4Xkev2gGp4HMJ9xagjm`](https://solscan.io/tx/5YoY4XZdgNvvQ8hMtYGHrzNdkS7MyabAg3FryvNReP6ufo2Hg7d9CfZpr3RyxAc37Fnjm4Xkev2gGp4HMJ9xagjm?cluster=devnet)
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: An account required by the instruction is missing. 
Logs: 
[
  "Program ComputeBudget111111111111111111111111111111 invoke [1]",
  "Program ComputeBudget111111111111111111111111111111 success",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH invoke [1]",
  "Program log: Instruction: BurnCrank",
  "Unknown program ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 34557 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: An account required by the instruction is missing"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T12:46:23.213Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:46:22.031Z
- deployer balance:  6.541321555 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`95Ba3HBdoTwHe8ZDBAU78BeutTWHfxdy6efRCz49WWgJKfcMTXKJiJJx1Ru5zLkLr2Epo9wq4Pd3rDPuj1zZPNZ`](https://solscan.io/tx/95Ba3HBdoTwHe8ZDBAU78BeutTWHfxdy6efRCz49WWgJKfcMTXKJiJJx1Ru5zLkLr2Epo9wq4Pd3rDPuj1zZPNZ?cluster=devnet)
- mint:  A1caJfJSWGVfSLLtMn1mSdmdmFsvDgT7P4LW2dV1mexA
- FAILED:  AnchorError thrown in src/buy.rs:57. Error Code: BuyZeroAmount. Error Number: 6020. Error Message: Buy zero amount.
Program log: Left: 0
Program log: Right: 0

## devnet 2026-10-30T12:51:49.716Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:51:49.308Z
- deployer balance:  6.523528995 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- curve after sell:  real sol 0
- curve complete:  true, real sol 0, team 6.523528995 sol, burn pda 0.015839887 sol
- FAILED:  AnchorError thrown in src/migrate.rs:109. Error Code: BondingCurveComplete. Error Number: 6005. Error Message: the curve is complete. anyone can call migrate to open its raydium pool..

## devnet 2026-10-30T12:52:01.867Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:52:01.348Z
- deployer balance:  6.523528995 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- curve after sell:  real sol 0
- curve complete:  true, real sol 0, team 6.523528995 sol, burn pda 0.015839887 sol
- migrate:  pool already exists, skipped
- pool:  C5S2LJWeDU2aeqvL1EYkhmpatApxNvrpkUhqVfsXQk8b (lp mint AfXe8e4xSrRUKk2Fttdpiy7JSvNWLacVHGhGhnLJBk6A), team 6.523528995 sol, burn pda 0.015839887 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: Cross-program invocation with unauthorized signer or writable account. 
Logs: 
[
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 112 of 500549 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [3]",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 105 of 498374 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb consumed 21160 of 518158 compute units",
  "Program DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb success",
  "23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC's writable privilege escalated",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 105921 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: Cross-program invocation with unauthorized signer or writable account"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T12:55:35.811Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T12:55:33.518Z
- deployer balance:  6.520093995 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- curve after sell:  real sol 0
- curve complete:  true, real sol 0, team 6.520093995 sol, burn pda 0.015839887 sol
- migrate:  pool already exists, skipped
- pool:  C5S2LJWeDU2aeqvL1EYkhmpatApxNvrpkUhqVfsXQk8b (lp mint AfXe8e4xSrRUKk2Fttdpiy7JSvNWLacVHGhGhnLJBk6A), team 6.520093995 sol, burn pda 0.015839887 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- burn_crank: [`STWeeTL4WUcWbgGh2yREnbbonZw544BSZz3KYH2E4wsahDXH4UtxUNfonMNyfeBrxeySjXwubvk4QeKQDjGEmzv`](https://solscan.io/tx/STWeeTL4WUcWbgGh2yREnbbonZw544BSZz3KYH2E4wsahDXH4UtxUNfonMNyfeBrxeySjXwubvk4QeKQDjGEmzv?cluster=devnet)
- burn:  burn pda 0.015839887 → 0.01148844 sol
- collect_creator_fee: [`8XFVHyAkecFo75TRo5CHf4bKgy8VSBJSDyFiJKaAZbT2mfYAMu9DpwVGAfqp9iD8mN52BCYgAsvUCs1XorCpt46`](https://solscan.io/tx/8XFVHyAkecFo75TRo5CHf4bKgy8VSBJSDyFiJKaAZbT2mfYAMu9DpwVGAfqp9iD8mN52BCYgAsvUCs1XorCpt46?cluster=devnet)
- done:  deployer 6.524468355 sol

## devnet 2026-10-30T13:18:00.665Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T13:17:55.032Z
- deployer balance:  6.521138355 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`3S6R53wwzRvs6PHNj81wFdiWRg1mW8S9zs22JSL4UdMBuWgvaRe1VwZ4a4Sm9TTHHAjSe5xiUPQedkdKNB6uddJV`](https://solscan.io/tx/3S6R53wwzRvs6PHNj81wFdiWRg1mW8S9zs22JSL4UdMBuWgvaRe1VwZ4a4Sm9TTHHAjSe5xiUPQedkdKNB6uddJV?cluster=devnet)
- mint:  FHhadx2gYZhszR3VAWyrzjseWnqE1TzcWfazqLDWBYbM
- buy 0.2 sol: [`2A9Hc8wRdJSwyPqUvxzRWBqja5Cu8aJmR9MdtgC7Fd8y8KhMxJS6gPiA5gQvVDPBdxQrYSXB1KGk34TaHdLGam4v`](https://solscan.io/tx/2A9Hc8wRdJSwyPqUvxzRWBqja5Cu8aJmR9MdtgC7Fd8y8KhMxJS6gPiA5gQvVDPBdxQrYSXB1KGk34TaHdLGam4v?cluster=devnet)
- curve after buy:  real sol 0.197238658, real tokens 367475250231199
- sell half: [`5jwjRQyyQyKAVr7futML7QVMMMYAXqhagwJbQ6xquKD6LcCtw8ntvkHmNapZcu5aaNfPm9VHijBm7einjddGPi2q`](https://solscan.io/tx/5jwjRQyyQyKAVr7futML7QVMMMYAXqhagwJbQ6xquKD6LcCtw8ntvkHmNapZcu5aaNfPm9VHijBm7einjddGPi2q?cluster=devnet)
- curve after sell:  real sol 0.074220684
- buy to complete #1: [`5A72MdUEj8M8jia5LMk1Bo3w8FXfSfDjLgC9xs4Zc15eZ4PPJYAwYSKqNd1JDKFuSMWFvCi2GAjW7JiQh7PkUCrv`](https://solscan.io/tx/5A72MdUEj8M8jia5LMk1Bo3w8FXfSfDjLgC9xs4Zc15eZ4PPJYAwYSKqNd1JDKFuSMWFvCi2GAjW7JiQh7PkUCrv?cluster=devnet)
- buy to complete #2: [`2wcd7RTCHrBgyPdQpAj93ebKhdrwpi3HfN1ikepotVR6GseVtoJV5yRcbqDPdaF3s15dJPyLZc3cfu7oseVVhajU`](https://solscan.io/tx/2wcd7RTCHrBgyPdQpAj93ebKhdrwpi3HfN1ikepotVR6GseVtoJV5yRcbqDPdaF3s15dJPyLZc3cfu7oseVVhajU?cluster=devnet)
- curve complete:  true, real sol 0.850053598, team 5.64191895 sol, burn pda 0.016968887 sol
- migrate: [`2hHvvrJK4nvAVWxtNjbeDp7mhtWNSvrxjkWiHaB93AseZyCStHH5n1uysjMShuRLjP7rEkHAKnHP2X4Ets2tkjWA`](https://solscan.io/tx/2hHvvrJK4nvAVWxtNjbeDp7mhtWNSvrxjkWiHaB93AseZyCStHH5n1uysjMShuRLjP7rEkHAKnHP2X4Ets2tkjWA?cluster=devnet)
- pool:  7vNMWZeybnmhAD6xa3MkxRxMtgXtMbL2cGexEvswuxj5 (lp mint Ecm8yhjue17Gxszj7BMz7ZWZmbfy8bZiAg1YZzkNnDBY), team 5.65227339 sol, burn pda 0.027328327 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- FAILED:  AnchorError caused by account: platform_coin. Error Code: ConstraintAddress. Error Number: 2012. Error Message: An address constraint was violated.
Program log: Left:
Program log: FHhadx2gYZhszR3VAWyrzjseWnqE1TzcWfazqLDWBYbM
Program log: Right:
Program log: 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC

## devnet 2026-10-30T13:27:08.107Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T13:27:05.738Z
- deployer balance:  5.65227339 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  FHhadx2gYZhszR3VAWyrzjseWnqE1TzcWfazqLDWBYbM
- curve after sell:  real sol 0
- curve complete:  true, real sol 0, team 5.65227339 sol, burn pda 0.027328327 sol
- migrate:  pool already exists, skipped
- pool:  7vNMWZeybnmhAD6xa3MkxRxMtgXtMbL2cGexEvswuxj5 (lp mint Ecm8yhjue17Gxszj7BMz7ZWZmbfy8bZiAg1YZzkNnDBY), team 5.65227339 sol, burn pda 0.027328327 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- burn_crank: [`3rHho5ejpbe7BAXjccSF81RSxmVeNoPRUJ5vQx4gP4gE8psqpaAYvdjibD5ZZMk7iTtMYydXwokNn687Ss9ghLjb`](https://solscan.io/tx/3rHho5ejpbe7BAXjccSF81RSxmVeNoPRUJ5vQx4gP4gE8psqpaAYvdjibD5ZZMk7iTtMYydXwokNn687Ss9ghLjb?cluster=devnet)
- burn:  platform coin 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC, burn pda 0.027328327 → 0.01148844 sol
- collect_creator_fee: [`3FxN8HSLKrzAfhj4M5cjjDYi7ANNFzJmsKY7h5rCGAoFTttMZE6prc289BkQUAEDWvyzs9WNSG15QFJweGRdC5hk`](https://solscan.io/tx/3FxN8HSLKrzAfhj4M5cjjDYi7ANNFzJmsKY7h5rCGAoFTttMZE6prc289BkQUAEDWvyzs9WNSG15QFJweGRdC5hk?cluster=devnet)
- done:  deployer 5.65664775 sol
