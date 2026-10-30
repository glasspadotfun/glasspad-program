
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

## devnet 2026-10-30T13:42:36.342Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T13:42:28.237Z
- deployer balance:  5.65331275 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`332vf5Fqxi3jWvjXyLBrQsnHhkwYQsFppZdJ4g56dqWjC697avbEXn2xcg3C7dcitU3Urd1FU2QZGfRo8KLWWAGd`](https://solscan.io/tx/332vf5Fqxi3jWvjXyLBrQsnHhkwYQsFppZdJ4g56dqWjC697avbEXn2xcg3C7dcitU3Urd1FU2QZGfRo8KLWWAGd?cluster=devnet)
- mint:  HZuswGsH5Y69hcbzNUDvLXp1HWPngAgUEMBeBaZPaccy
- buy 0.2 sol: [`2u7AcdQP4jCiMouBqBCo2a7AMe1Ebb829osxh185RJHB8AmfnahgdVfb19njAPYojG1XfQxF1N4nDMeSGSiMdEzG`](https://solscan.io/tx/2u7AcdQP4jCiMouBqBCo2a7AMe1Ebb829osxh185RJHB8AmfnahgdVfb19njAPYojG1XfQxF1N4nDMeSGSiMdEzG?cluster=devnet)
- curve after buy:  real sol 0.197238658, real tokens 367475250231199
- sell half: [`2BxPTkLvBc3AxndKUBTyZpA7snMWhrkJ7im7Sx1JPC7kLCMoiZySmRhvVswEoqkvTDPzQUMtQFxzPNL2PcTypTst`](https://solscan.io/tx/2BxPTkLvBc3AxndKUBTyZpA7snMWhrkJ7im7Sx1JPC7kLCMoiZySmRhvVswEoqkvTDPzQUMtQFxzPNL2PcTypTst?cluster=devnet)
- curve after sell:  real sol 0.074220684
- buy to complete #1: [`4jdW2FV8Z2VPHxc1DFiHz6TA2JD2ZvqH7fV22WPP4SmCjsWYt3rDDtyxYRJrfipg9XdKQLuk1TP88nS8giT1s5H2`](https://solscan.io/tx/4jdW2FV8Z2VPHxc1DFiHz6TA2JD2ZvqH7fV22WPP4SmCjsWYt3rDDtyxYRJrfipg9XdKQLuk1TP88nS8giT1s5H2?cluster=devnet)
- buy to complete #2: [`4DLe26BhRRNN1XyAsKcXicP1Lx6wnoU3A2MXSnT61QcTXXnPTr1xBAHoGYUFRNQNDwK71NTjtTjfFsYjhcHvKrdU`](https://solscan.io/tx/4DLe26BhRRNN1XyAsKcXicP1Lx6wnoU3A2MXSnT61QcTXXnPTr1xBAHoGYUFRNQNDwK71NTjtTjfFsYjhcHvKrdU?cluster=devnet)
- curve complete:  true, real sol 0.850053598, team 4.774093345 sol, burn pda 0.016968887 sol
- migrate: [`53E8hHWQ1ytt9zSzMU957pdhzPKshuk1W9v2GEWAqouah7ALgHDDUK2ekUz4pkACbjMutKT8zbbskSUCWCkh7CzA`](https://solscan.io/tx/53E8hHWQ1ytt9zSzMU957pdhzPKshuk1W9v2GEWAqouah7ALgHDDUK2ekUz4pkACbjMutKT8zbbskSUCWCkh7CzA?cluster=devnet)
- pool:  4mc7z2nneqoox4JMsTyBTrUU5QCyKvkTtjwZt6ZCD1EC (lp mint 8Xs3y5FzkKXx43q5cfFLyNLtQuv1ynva4DF3f68785G4), team 4.784447785 sol, burn pda 0.027328327 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- burn_crank: [`36jbBnaPT1FKi4875Zm7ZYka18GF6B7yJFJmR8PU4oGBjLB3o4SSvpJ6Hp6CJstW2Zdkt11QRY5pCafv7gygynkD`](https://solscan.io/tx/36jbBnaPT1FKi4875Zm7ZYka18GF6B7yJFJmR8PU4oGBjLB3o4SSvpJ6Hp6CJstW2Zdkt11QRY5pCafv7gygynkD?cluster=devnet)
- burn:  platform coin 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC, burn pda 0.027328327 → 0.01148844 sol
- collect_creator_fee: [`5WyuQLYPDgcoeHbccyeHKKxLnTVoTUQiBSbBeCtQz88uAvY4BeRNpRxG9GHrkJPrM3zdaaaKxKUpUYVyzRSawZgM`](https://solscan.io/tx/5WyuQLYPDgcoeHbccyeHKKxLnTVoTUQiBSbBeCtQz88uAvY4BeRNpRxG9GHrkJPrM3zdaaaKxKUpUYVyzRSawZgM?cluster=devnet)
- done:  deployer 4.788822145 sol

## devnet 2026-10-30T13:59:24.946Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T13:59:20.437Z
- deployer balance:  4.785487145 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`hEeYCFNEvC1grPfFM6nWoEax1yU3tYFBoeZ9uKArQVS71AdcesLPnkNnJwtszCyoMifQw6HaPj6dGNP39vsvRvY`](https://solscan.io/tx/hEeYCFNEvC1grPfFM6nWoEax1yU3tYFBoeZ9uKArQVS71AdcesLPnkNnJwtszCyoMifQw6HaPj6dGNP39vsvRvY?cluster=devnet)
- mint:  8tWxP6wGdMwXuRorce4yvkchrCb74yGGcinX5P3twLr6
- buy 0.2 sol: [`L7crbWfT7CTyWAeenWB8JdEX9xoq1xrCid1tQxXg19VPhbosjuA3cQY2DU1WYGB6QsFVxoJoWF5ECEbLV7XMA39`](https://solscan.io/tx/L7crbWfT7CTyWAeenWB8JdEX9xoq1xrCid1tQxXg19VPhbosjuA3cQY2DU1WYGB6QsFVxoJoWF5ECEbLV7XMA39?cluster=devnet)
- curve after buy:  real sol 0.197238658, real tokens 367475250231199
- sell half: [`6L8LyUGLhDBMQgGAH7jufmnJUrJBMfFXdv8WNBnPXqYgVTXCQqGjpRhXFDZszEqnFK55q7okqUxDEd1F7emwX9x`](https://solscan.io/tx/6L8LyUGLhDBMQgGAH7jufmnJUrJBMfFXdv8WNBnPXqYgVTXCQqGjpRhXFDZszEqnFK55q7okqUxDEd1F7emwX9x?cluster=devnet)
- curve after sell:  real sol 0.074220684
- buy to complete #1: [`4QnCTLqcpm6NcrtoEUy8kfSDUhXELjNtKguPX7RrLLYsqh8LuBTWzuRbzbHsGZMVBc1yNhxB3TuMwtMXPTQKb2C`](https://solscan.io/tx/4QnCTLqcpm6NcrtoEUy8kfSDUhXELjNtKguPX7RrLLYsqh8LuBTWzuRbzbHsGZMVBc1yNhxB3TuMwtMXPTQKb2C?cluster=devnet)
- buy to complete #2: [`4DHYPyiZZUcBWLvSzJke471LWEnLLnx4d6S62Q8DiUo9BvCGUiUiyGFxqzk5KPMgVM2w7g4ir7Yct8DD3QvPAuER`](https://solscan.io/tx/4DHYPyiZZUcBWLvSzJke471LWEnLLnx4d6S62Q8DiUo9BvCGUiUiyGFxqzk5KPMgVM2w7g4ir7Yct8DD3QvPAuER?cluster=devnet)
- curve complete:  true, real sol 0.850053598, team 3.90626774 sol, burn pda 0.016968887 sol
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: Program failed to complete. 
Logs: 
[
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 118 of 402804 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program 11111111111111111111111111111111 invoke [2]",
  "Program 11111111111111111111111111111111 success",
  "Program 11111111111111111111111111111111 invoke [2]",
  "Program 11111111111111111111111111111111 success",
  "Program data: jXqFaat97W9RK+b3nLza4AAChXGE8l37TaIjscLRjCcTGLcgHholRHU0TL+toxT+97RPnhqc5BYo4vtmx1XTYm0GbDRiln4rAAgBqSy8AADeB78mAAAAAADC6wsAAAAAkBKeAAAAAACQEp4AAAAAAP+RU2pPj7T1vqSy00Ymf96pzfLd0cB2U+XzyZgLyT8Ya9E7Y1UAAAC8ouRqAAAAAA==",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 202404 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: Access violation reading 8 bytes at address 0xfe14a3adbf000000 (in unallocated region)"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T13:59:37.542Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T13:59:37.127Z
- deployer balance:  3.90626774 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- mint:  8tWxP6wGdMwXuRorce4yvkchrCb74yGGcinX5P3twLr6
- curve after sell:  real sol 0.850053598
- curve complete:  true, real sol 0.850053598, team 3.90626774 sol, burn pda 0.016968887 sol
- FAILED:  Simulation failed. 
Message: Transaction simulation failed: Error processing Instruction 1: Program failed to complete. 
Logs: 
[
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA invoke [2]",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA consumed 118 of 402804 compute units",
  "Program TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA success",
  "Program 11111111111111111111111111111111 invoke [2]",
  "Program 11111111111111111111111111111111 success",
  "Program 11111111111111111111111111111111 invoke [2]",
  "Program 11111111111111111111111111111111 success",
  "Program data: jXqFaat97W9RK+b3nLza4AAChXGE8l37TaIjscLRjCcTGLcgHholRHU0TL+toxT+97RPnhqc5BYo4vtmx1XTYm0GbDRiln4rAAgBqSy8AADeB78mAAAAAADC6wsAAAAAkBKeAAAAAACQEp4AAAAAAP+RU2pPj7T1vqSy00Ymf96pzfLd0cB2U+XzyZgLyT8Ya9E7Y1UAAADIouRqAAAAAA==",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH consumed 202404 of 599850 compute units",
  "Program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH failed: Access violation reading 8 bytes at address 0xfe14a3adbf000000 (in unallocated region)"
]. 
Catch the `SendTransactionError` and call `getLogs()` on it for full details.

## devnet 2026-10-30T14:05:07.363Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T14:04:59.807Z
- deployer balance:  3.90293774 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`2EqW5uxjZ4sG1dStaKAsRE1hkoaqfmGoSoAg4SwuGK3mQTyanXvx9KDhqjg48tBpP5oUJ1DfjjisAW4p9jVNTrbP`](https://solscan.io/tx/2EqW5uxjZ4sG1dStaKAsRE1hkoaqfmGoSoAg4SwuGK3mQTyanXvx9KDhqjg48tBpP5oUJ1DfjjisAW4p9jVNTrbP?cluster=devnet)
- mint:  9Ubfzo5bayHcAZ46aiinsF86KUTVfVsYrfz3NMirco34
- buy 0.2 sol: [`37qiYcvNw6suuAxZ4FhvYfGUCLvFuhzzwLUdUY5Mk8P2B7PFocT7r4vqSYV6nN7rrQyFGYZYcq1F29duXimPWFVA`](https://solscan.io/tx/37qiYcvNw6suuAxZ4FhvYfGUCLvFuhzzwLUdUY5Mk8P2B7PFocT7r4vqSYV6nN7rrQyFGYZYcq1F29duXimPWFVA?cluster=devnet)
- curve after buy:  real sol 0.197238658, real tokens 367475250231199
- sell half: [`5epqq9zg7mAMRSky3JSS2dzTsyBBErPDcG7cWoFNdycXwTFctGFybHeuRYvt7B29MarcopWXZ3whSqHA4TVdx9Vv`](https://solscan.io/tx/5epqq9zg7mAMRSky3JSS2dzTsyBBErPDcG7cWoFNdycXwTFctGFybHeuRYvt7B29MarcopWXZ3whSqHA4TVdx9Vv?cluster=devnet)
- curve after sell:  real sol 0.074220684
- buy to complete #1: [`5WXNNdHqrVZfKAtSvPM7tBmMnbS2Qp9UicVrxT6waCQdUPbtaesbfsnvCvHtgMT7VDmAueyAh1onSGJhm9xgy54w`](https://solscan.io/tx/5WXNNdHqrVZfKAtSvPM7tBmMnbS2Qp9UicVrxT6waCQdUPbtaesbfsnvCvHtgMT7VDmAueyAh1onSGJhm9xgy54w?cluster=devnet)
- buy to complete #2: [`23jQfbeLi7v1gXPbxPmukRkGXLvydHxRkjNP3dL7v1AefvAR9m9UoNFW8xHXRukAu2VvopER9dmCJu7pAohoPA2y`](https://solscan.io/tx/23jQfbeLi7v1gXPbxPmukRkGXLvydHxRkjNP3dL7v1AefvAR9m9UoNFW8xHXRukAu2VvopER9dmCJu7pAohoPA2y?cluster=devnet)
- curve complete:  true, real sol 0.850053598, team 3.023718335 sol, burn pda 0.022449334 sol
- migrate: [`5qhuPXWz3XdWDY3Yn5aXgRypHefprFJKXujhzugFXShkZ7cifX36dRfQnUP52qXh4t12i5AgMAhn1DVs4Hrnuox7`](https://solscan.io/tx/5qhuPXWz3XdWDY3Yn5aXgRypHefprFJKXujhzugFXShkZ7cifX36dRfQnUP52qXh4t12i5AgMAhn1DVs4Hrnuox7?cluster=devnet)
- pool:  2ZXpNbjEcWvjfbthA5BJdnNmTEZFVaeRHrWVDPcr4LqY (lp mint DQSNAu7oJchYr6DbNaEqhXYcMDEU4kjvvYw2HaCKEkDX), team 3.034072775 sol, burn pda 0.032808774 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- burn_crank: [`3hbf5P4aJhdGzeSN8AiuRHHkhgU5ATLGCnumf6phQvi7mncNAUURZg3iJV5fQXjuaBHzppNbjnFmmjbPwojtcoAV`](https://solscan.io/tx/3hbf5P4aJhdGzeSN8AiuRHHkhgU5ATLGCnumf6phQvi7mncNAUURZg3iJV5fQXjuaBHzppNbjnFmmjbPwojtcoAV?cluster=devnet)
- burn:  platform coin 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC, burn pda 0.032808774 → 0.01148844 sol
- collect_creator_fee: [`3YVa61c64eLaBUnnfNzD3SzLti2SZ8zXzZmzqJ9o967WPE5gkaWcPHZEn5fvmwPDZdZLR7PajAJT4qN1kdhMCMyA`](https://solscan.io/tx/3YVa61c64eLaBUnnfNzD3SzLti2SZ8zXzZmzqJ9o967WPE5gkaWcPHZEn5fvmwPDZdZLR7PajAJT4qN1kdhMCMyA?cluster=devnet)
- done:  deployer 3.042831495 sol

## devnet 2026-10-30T15:19:31.736Z
- canary on devnet:  program DpB6AVXBToz1wqYURvpttSThxcFHyzWe71F1Ufg1escH, deployer 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, 2026-10-30T15:18:43.423Z
- deployer balance:  3.641226019 sol
- initialize:  global already exists, skipped
- global:  fee 100 bps, creator 40 bps, buyback 5000 bps, migration fee 0.2 sol, team 6TrrMJAHMriYKHAak9xwL2TqCLWLW7kJwM2A7ackmqfy, burn 2P5zbfTUDNsaqCKaPZjLmUo5ezkHpPANqiRhHDioySdw
- create: [`uS7NfJLoB9kqLe5aKod83ZphGyXPTv4CHEUY1cFaFKM6eJnTdNYYnePr7zoj8xkVr8eXk3BewF7haEVRy4LqzZz`](https://solscan.io/tx/uS7NfJLoB9kqLe5aKod83ZphGyXPTv4CHEUY1cFaFKM6eJnTdNYYnePr7zoj8xkVr8eXk3BewF7haEVRy4LqzZz?cluster=devnet)
- mint:  9Rv6tjMZdY1BBJbmb4hzA7q5aKhEPSL7PBy7wvApA79f
- buy 0.2 sol: [`2vBU4JDYMxN25dmB1V4PxGZw7iuteBi7f9v6SCg6wobpkP6EyJtcptQ2czzaBMSkx4yX6uxz2MwiT5ReK8VhMdEm`](https://solscan.io/tx/2vBU4JDYMxN25dmB1V4PxGZw7iuteBi7f9v6SCg6wobpkP6EyJtcptQ2czzaBMSkx4yX6uxz2MwiT5ReK8VhMdEm?cluster=devnet)
- curve after buy:  real sol 0.197238658, real tokens 367475250231199
- sell half: [`2Cf4iFfa3AhiTW3g4diDFUrb5ZDAgS6D8k9jz5FCFuAGkHkuGfnSyZwN9mu9koyXFFT3pE61Vq14Pyn9UmrhVWqM`](https://solscan.io/tx/2Cf4iFfa3AhiTW3g4diDFUrb5ZDAgS6D8k9jz5FCFuAGkHkuGfnSyZwN9mu9koyXFFT3pE61Vq14Pyn9UmrhVWqM?cluster=devnet)
- curve after sell:  real sol 0.074220684
- buy to complete #1: [`5JG4gTc2AQsjbPCYNhRG47M4rSP481MT4fZGLXgzcuGkt5QjmhNMFSMwGSZ7A3ibrrUZGig2VpRR7BRnsk7DbGFj`](https://solscan.io/tx/5JG4gTc2AQsjbPCYNhRG47M4rSP481MT4fZGLXgzcuGkt5QjmhNMFSMwGSZ7A3ibrrUZGig2VpRR7BRnsk7DbGFj?cluster=devnet)
- buy to complete #2: [`H9FH4ETLRGbcbvSjmMRM2dKv2PAvjJUCsziGjrNKcx1DMJikvGFNeuvGjmPqBjeaVYVpAT3HFBhMZSUv3sz1xiu`](https://solscan.io/tx/H9FH4ETLRGbcbvSjmMRM2dKv2PAvjJUCsziGjrNKcx1DMJikvGFNeuvGjmPqBjeaVYVpAT3HFBhMZSUv3sz1xiu?cluster=devnet)
- curve complete:  true, real sol 0.850053598, team 2.762006614 sol, burn pda 0.017348071 sol
- migrate: [`24tpu8nvMQdAQHQ8wiHEUAKk6Nnz5uFJ2uuXdiV1Y4UcGcvM7V3Fx8qw9AJEt87uD65hd3xcLmvBStEctGHhm26d`](https://solscan.io/tx/24tpu8nvMQdAQHQ8wiHEUAKk6Nnz5uFJ2uuXdiV1Y4UcGcvM7V3Fx8qw9AJEt87uD65hd3xcLmvBStEctGHhm26d?cluster=devnet)
- pool:  F3b8RHKnwRDRNApCra52QUk7N3Fq9xgKJp8i4m5LvbuL (lp mint Ea6ac23Z6XAGeMJQb87dS6hKV2LXPeXsGxSTTvrfqSj), team 2.772361054 sol, burn pda 0.027707511 sol
- seal_platform_coin:  already sealed to 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC
- burn_crank: [`AK1aPHkgnWz8PzKHRaAdHtd5oEzrj1cYWw9HyBjG4Tqn9e6A1UsF7PnDVpYCwK12V5mbXwfVjsSsuR7ZVCnaiVK`](https://solscan.io/tx/AK1aPHkgnWz8PzKHRaAdHtd5oEzrj1cYWw9HyBjG4Tqn9e6A1UsF7PnDVpYCwK12V5mbXwfVjsSsuR7ZVCnaiVK?cluster=devnet)
- burn:  platform coin 23enffH6dZ5MnGAJCebMozdyBjNh1RYYiSN7LAn1QKmC, burn pda 0.027707511 → 0.01148844 sol
- collect_creator_fee: [`4EoBgpDMMjzdQheuxEasqSjcFtK16pA2cmBYeFomoCZWCTD7XibZd713gFBb4B1Wgf7ziLQYWRFKHY7mXjQXjpmd`](https://solscan.io/tx/4EoBgpDMMjzdQheuxEasqSjcFtK16pA2cmBYeFomoCZWCTD7XibZd713gFBb4B1Wgf7ziLQYWRFKHY7mXjQXjpmd?cluster=devnet)
- done:  deployer 2.777038762 sol
