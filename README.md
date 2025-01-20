# whirlpool-stream-client
Whirlpool Stream Client library is a client library for [Whirlpool Stream](https://github.com/yugure-orca/whirlpool-stream-doc).

## Rust
## Examples
### helloworld-stream
The simplest example to connect to the Whirlpool Stream and print received messages.

```bash
cargo run -p helloworld-stream
```

You can use your own APIKEY if you have one.
```bash
cargo run -p helloworld-stream -- -k YOUR_APIKEY
```

Example output:
```
Data: slot:315247506, height:293547273, time:1737390075 (7s ago), events:5, accounts:6
Data: slot:315247507, height:293547274, time:1737390075 (7s ago), events:15, accounts:14
Data: slot:315247508, height:293547275, time:1737390076 (6s ago), events:17, accounts:19
Data: slot:315247509, height:293547276, time:1737390076 (6s ago), events:12, accounts:13
Data: slot:315247510, height:293547277, time:1737390077 (7s ago), events:10, accounts:10
Data: slot:315247511, height:293547278, time:1737390077 (7s ago), events:12, accounts:10
```

### monitor-trade
The example to monitor ALL trades on Whirlpool.

```bash
cargo run -p monitor-trade
```

Example output:
```
Data: slot:315247965, height:293547732, time:1737390261
        Traded pool: CdT9SCJk2uSrDipxAkNpYbpbxn7Nbuy6zX1MXxN53QXa, direction: AtoB, in: 48761453, out: 22251382908, price: 0.4660081970 -> 0.4652807101, payer: BP8P9ewuPiAeAf31ykYVAxJXWY3KtQNg5JoakUZ2HUgG
        Traded pool: 7qbRF6YsyGuLUVs6Y1q64bdVrfe4ZcUUz1JRdoVNUJnm, direction: BtoA, in: 10783059, out: 43482982, price: 247.8593238 -> 247.8595237, payer: BEmUSjqs7mpgaSXw6QdrePfTsD8aQHbdtnqUxa63La6E
        Traded pool: Av2BjqE8ZuAUbepswPBGTPCrJhdpFtNs3FTai1yt9QnA, direction: AtoB, in: 47222611, out: 247056, price: 5.235566315 -> 5.232125417, payer: nov7uFcW8PSdWsKLQVjW3gJntWMYzS4zBzGkX9ChxVr
        Traded pool: AyFajbj7QEi8CizFnfEjJn3vSUxgDjVKob4A8i618YJD, direction: AtoB, in: 27911179, out: 26239438, price: 0.9430243724 -> 0.9428432562, payer: 8Kj2MoiGtDDLFZMPCCAgGkRQ2RFi2QV2YQ4K1EGvhMTz
        Traded pool: FTZXgbCYGnVEVMHCTWg6w9YqFiNdjd4wpmEuydf1d7f1, direction: BtoA, in: 1895163960, out: 1128294394, price: 0.001674824975 -> 0.001679145898, payer: 3LkGTjNsF2zWc2ddBPHYyEJJZKqbdHJDgfjztxnjwL5R
        Traded pool: FTZXgbCYGnVEVMHCTWg6w9YqFiNdjd4wpmEuydf1d7f1, direction: AtoB, in: 60000000, out: 100580671, price: 0.001679145898 -> 0.001678916070, payer: 7LHTrNJbiXb7ayDgZeScHnwBRYGnMz4MfBkyhwe7czvu
        Traded pool: 9YyNqgM3Wcjnzb4DXiyNBbcLEkGAyaiB4PSeQyZsFtr8, direction: AtoB, in: 310000000, out: 4978170485, price: 161.1003316 -> 161.0383799, payer: 8Kj2MoiGtDDLFZMPCCAgGkRQ2RFi2QV2YQ4K1EGvhMTz
        Traded pool: HktfL7iwGKT5QHjywQkcDnZXScoh811k7akrMZJkCcEF, direction: BtoA, in: 567927, out: 76332316, price: 0.07436455226 -> 0.07436485312, payer: AmbcqD1BDrREBGHtRgcCjVe5JsEFTQ8T2r4Hi82ETP9e
        Traded pool: AU971DrPyhhrpRnmEBp5pDTWL2ny7nofb5vYBjDJkR2E, direction: AtoB, in: 1487287, out: 49602080, price: 3336.752799 -> 3336.727920, payer: r6ctPq1h6cwfnVVVuqUS8qJ8MLormW22ZMxBfMQ88aN
        Traded pool: HD8i7qr1hd9ida6sN71RbkLxbWcbvZS4NA5CY6vfcDpj, direction: BtoA, in: 49602080, out: 10473008, price: 4.733726033 -> 4.733902492, payer: r6ctPq1h6cwfnVVVuqUS8qJ8MLormW22ZMxBfMQ88aN
        Traded pool: 7rCWunLK7a2uLYog5e79Nxz5gfXptJaotjBuqv29JKz, direction: AtoB, in: 10473008, out: 47395, price: 0.00004528267313 -> 0.00004527233682, payer: r6ctPq1h6cwfnVVVuqUS8qJ8MLormW22ZMxBfMQ88aN
        Traded pool: B5EwJVDuAauzUEEdwvbuXzbFFgEYnUqqS37TUM1c4PQA, direction: BtoA, in: 47395, out: 200054788, price: 0.002367896459 -> 0.002367906202, payer: r6ctPq1h6cwfnVVVuqUS8qJ8MLormW22ZMxBfMQ88aN
Data: slot:315247966, height:293547733, time:1737390262
        Traded pool: HZsTF6VHdQy2W6cfEEqqpoTFKocx7Ch5c4TnWucXkAYv, direction: BtoA, in: 704727600, out: 656348083, price: 1.073684080 -> 1.073716896, payer: winbxV2N4uoHxbb8zE7qeGMks7P3AYAUKvczFwWeq11
        Traded pool: Hp53XEtt4S8SvPCXarsLSdGfZBuUr5mMmZmX2DRNXQKp, direction: BtoA, in: 705103120, out: 824449376, price: 0.8551557274 -> 0.8551557349, payer: winbxV2N4uoHxbb8zE7qeGMks7P3AYAUKvczFwWeq11
        Traded pool: fcdyWYBQcccmTG11V9nVjqHNfpg32aVGCknfzcuhhtF, direction: BtoA, in: 108634680, out: 31462423, price: 3447.125387 -> 3447.503389, payer: BX8Xe9xvq5xdrjss2JqjzYxPzSweU3pXqb2W31tH9mXP
        Traded pool: FpCMFDFGYotvufJ7HrFHsWEiiQCGbkLCtwHiDnh7o28Q, direction: AtoB, in: 2015495138, out: 499655294, price: 247.9924684 -> 247.9206610, payer: YubQzu18FDqJRyNfG8JqHmsdbxhnoQqcKUHBdUkN6tP
        Traded pool: DVwtABQwmjx9DosuZWGkAv1kiibgAAPoaFJB3jTHfo2V, direction: AtoB, in: 223045730, out: 63039159446, price: 283.3927545 -> 282.7711620, payer: 9osj5ksZcrYVeft5pjN1KUNpuM9xiZpjLPxtiMHG4AT5
        Traded pool: 25q3qfHDpRq53HjPsxYEqnLg47rbMH5JiwTRY8HX9mP9, direction: AtoB, in: 63039159446, out: 65529687, price: 1.040045067 -> 1.040010241, payer: 9osj5ksZcrYVeft5pjN1KUNpuM9xiZpjLPxtiMHG4AT5
        Traded pool: FTZXgbCYGnVEVMHCTWg6w9YqFiNdjd4wpmEuydf1d7f1, direction: AtoB, in: 48996909, out: 82125487, price: 0.001678916070 -> 0.001678728423, payer: nov9zXXSrm5ZuDhcWETNvk1HgFtanhGDHSyzF6Mq4Xi
        Traded pool: CKyPqtvkenW8SRYsK4z5txZzCsCZFS2NTCrzLMmtXTPz, direction: BtoA, in: 47999189, out: 15243607, price: 3.143593746 -> 3.143945507, payer: 4ZSAjZH6HDGTABPG5mnzVnLxcVgzYwLyiFf31MXQTMm3
        Traded pool: 83v8iPyZihDEjDdY8RdZddyZNyUtXngz69Lgo9Kt5d6d, direction: BtoA, in: 15243607, out: 61480986, price: 247.9119743 -> 247.9187790, payer: 4ZSAjZH6HDGTABPG5mnzVnLxcVgzYwLyiFf31MXQTMm3
        Traded pool: A2J7vmG9xAdWUzYscN7oQssxZBFihwD3UonkWB8Kod1A, direction: AtoB, in: 3526239, out: 24796952, price: 7.104145343 -> 7.102169950, payer: piDRvJrTjrWyQT2G6FVTYLucrRXR5WSZiWe6UdDRjYZ
        Traded pool: 55BrDTCLWayM16GwrMEQU57o4PTm6ceF9wavSdNZcEiy, direction: AtoB, in: 13875, out: 14518250, price: 104689.5582 -> 104688.1510, payer: AmbcqD1BDrREBGHtRgcCjVe5JsEFTQ8T2r4Hi82ETP9e
        Traded pool: 8phK65jxmTPEN158xLgSr4oZvssw9SyTErpNZj3g7px4, direction: AtoB, in: 48381508, out: 40871827, price: 0.8448665035 -> 0.8448664971, payer: 5y5jCvdD6uTPjWbQtLj8ABQ56jCor2SxURidAqGSyviw
        Traded pool: CwZbEdMZdxjnPLcRGRz8PwuvA4tK4iBmS9YZrMvnrNJr, direction: AtoB, in: 40871827, out: 38518958, price: 0.9425272904 -> 0.9425272548, payer: 5y5jCvdD6uTPjWbQtLj8ABQ56jCor2SxURidAqGSyviw
        Traded pool: 7GF7nYq7Efxn92Q9oDoHB5RrNVk2sECThCfjHUrgSLmx, direction: BtoA, in: 704524430, out: 597617927, price: 1.166949143 -> 1.167248537, payer: 8bJdFEsuoFsz5eiE2QKvdbsptpH6aUMg3HSEY23VNiNJ
        Traded pool: FpCMFDFGYotvufJ7HrFHsWEiiQCGbkLCtwHiDnh7o28Q, direction: AtoB, in: 1716414763, out: 425397122, price: 247.9206610 -> 247.8595336, payer: MfDuWeqSHEqTFVYZ7LoexgAK9dxk7cy4DFJWjWMGVWa
```

### monitor-liquidity
The example to monitor ALL liquidity operations (Deposit and Withdraw) on Whirlpool.

```bash
cargo run -p monitor-liquidity
```

Example output:
```
Data: slot:315248157, height:293547924, time:1737390340
        Deposited pool: 4fuUiYxTQ6QCrdSq9ouBYcTM7bqSwYTSyLueGZLTy4T4, range: [-16, 14], liquidity: 574863391756, a: 56593951, b: 805541296, payer: 2bhkQ6uVn32ddiG4Fe3DVbLsrExdb3ubaY6i1G4szEmq
Data: slot:315248158, height:293547925, time:1737390340
        Withdrawn pool: Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE, range: [-13548, -13508], liquidity: 504380140484, a: 1983848179, b: 0, payer: GtomzVbjyaqydpiKWUM1PE9uMr11d7Zhax9zt4bTMXqr
Data: slot:315248159, height:293547926, time:1737390340
Data: slot:315248160, height:293547927, time:1737390341
Data: slot:315248161, height:293547928, time:1737390341
Data: slot:315248162, height:293547929, time:1737390342
Data: slot:315248163, height:293547930, time:1737390342
Data: slot:315248164, height:293547931, time:1737390342
Data: slot:315248165, height:293547932, time:1737390343
Data: slot:315248166, height:293547933, time:1737390343
Data: slot:315248167, height:293547934, time:1737390343
        Deposited pool: 6d4UYGAEs4Akq6py8Vb3Qv5PvMkecPLS1Z9bBCcip2R7, range: [-13824, -11328], liquidity: 538577679, a: 126122832, b: 0, payer: EBA5cUAzvCa549VFiB8JnnHMSxrM1Y6dZDSaVuq827bW
Data: slot:315248168, height:293547935, time:1737390344
Data: slot:315248169, height:293547936, time:1737390344
        Withdrawn pool: 4q1d8xiftXysa8asehv9dC4ELYxBCmGT1zkiZGZR5h11, range: [-37376, -35328], liquidity: 1051574746, a: 0, b: 17497415, payer: 27g9gjTsz7qCgDJg2rgrBZ3e1oaac3d843b6iLpyViG5
Data: slot:315248170, height:293547937, time:1737390345
Data: slot:315248171, height:293547938, time:1737390345
Data: slot:315248172, height:293547939, time:1737390346
        Deposited pool: 6nD6d8gG17wakW6Wu5URktBZQp3uxp5orgPa576QXigJ, range: [37408, 41504], liquidity: 214309246, a: 4073369, b: 91654397, payer: DjGTf1tER8BVLmq492YZaykAARGMQAYMmY4k5RUg7ePk
        Withdrawn pool: 4q1d8xiftXysa8asehv9dC4ELYxBCmGT1zkiZGZR5h11, range: [-46592, -35072], liquidity: 468504618, a: 0, b: 35521388, payer: 59W989q7zaTRsCQiktYfdiCAaw4fBS2sHV1AExyezeZu
        Deposited pool: 6d4UYGAEs4Akq6py8Vb3Qv5PvMkecPLS1Z9bBCcip2R7, range: [-14400, -12480], liquidity: 579114974, a: 80844457, b: 6807197, payer: 41dNCB9nxtG6ZVEwocd2z1tTDZ8GxMVT9J3MShBer3cL
        Deposited pool: Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE, range: [-14108, -13700], liquidity: 756695298966, a: 16373863780, b: 3586381706, payer: 3bsmM2YgxEkjByojtkG72FoWrEvBvJtPVG3xmPwUtTjg
Data: slot:315248173, height:293547940, time:1737390346
Data: slot:315248174, height:293547941, time:1737390347
        Deposited pool: 21gTfxAnhUDjJGZJDkTXctGFKT8TeiXx6pN1CEg9K1uW, range: [-16080, -13952], liquidity: 119941875305, a: 0, b: 6026200457, payer: DyHaRUxuj6cqGmgHxDHfNjmrPpDsR6E1VCaCeh782NRr
```

### monitor-sol-price
The example to monitor SOL price on one of SOL/USDC whirlpools.

```bash
cargo run -p monitor-sol-price
```

Example output:
```
Data: slot:315248364, height:293548131, time:1737390424
        SOL price (🔥): 249.1118563, 27.440098555 SOL has been bought
Data: slot:315248365, height:293548132, time:1737390424
        SOL price (🔥): 249.1159905, 8.439999853 SOL has been bought
        SOL price (🔥): 249.1190204, 6.185304835 SOL has been bought
        SOL price (🔥): 249.1215923, 5.250288987 SOL has been bought
        SOL price (🔥): 249.1233998, 3.689737537 SOL has been bought
        SOL price (🔥): 249.1248538, 2.968222514 SOL has been bought
        SOL price (🔥): 249.1261035, 2.551081067 SOL has been bought
Data: slot:315248366, height:293548133, time:1737390425
Data: slot:315248367, height:293548134, time:1737390425
        SOL price (🔥): 249.1751463, 100.098318795 SOL has been bought
Data: slot:315248368, height:293548135, time:1737390426
Data: slot:315248369, height:293548136, time:1737390426
Data: slot:315248370, height:293548137, time:1737390426
Data: slot:315248371, height:293548138, time:1737390427
Data: slot:315248372, height:293548139, time:1737390427
        SOL price (💧): 249.1653219, 20.057784657 SOL has been sold
        SOL price (💧): 249.1554980, 20.057784657 SOL has been sold
Data: slot:315248373, height:293548140, time:1737390427
Data: slot:315248374, height:293548141, time:1737390428
Data: slot:315248375, height:293548142, time:1737390428
        SOL price (💧): 249.1456739, 20.059366395 SOL has been sold
Data: slot:315248376, height:293548143, time:1737390429
```

### monitor-new-pool
The example to monitor new whirlpool initializations.

```bash
cargo run -p monitor-new-pool
```

Example output:
```
Data: slot:315249695, height:293549462, time:1737390968
Data: slot:315249696, height:293549463, time:1737390968
        New pool: address: AbR4RvqCBuzPZPKRqqPTasKHUbLL1Nu97e68y6fmM5nT, config: 2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ, a: So11111111111111111111111111111111111111112, b: EtFqiFQwXvSwwNK1TRJ2CBQfQHKYw6yNPybXDSL9vkYG, tick_spacing: 32896, initial_price: 200000.0000, payer: 5DMHZTmxc8128Ad5aZ5qPurmQtz5jDQZgfkSgopAt6fK
Data: slot:315249697, height:293549464, time:1737390969
Data: slot:315249698, height:293549465, time:1737390969
```

### with-reconnect
The example to connect to Whirlpool Stream with reconnection feature and HA feature.

```bash
cargo run -p with-reconnect
```

Example output:
```
Connecting to endpoint: wss://orcanauts-a.whirlpool-stream.pleiades.dev slot: None
Data: slot:315249984, height:293549751, time:1737391087 (8s ago), events:10, accounts:14
Data: slot:315249985, height:293549752, time:1737391087 (8s ago), events:26, accounts:33
Data: slot:315249986, height:293549753, time:1737391088 (7s ago), events:21, accounts:21
Data: slot:315249987, height:293549754, time:1737391088 (7s ago), events:27, accounts:28
Data: slot:315249988, height:293549755, time:1737391088 (7s ago), events:10, accounts:10
Data: slot:315249989, height:293549756, time:1737391089 (7s ago), events:17, accounts:23
Data: slot:315249990, height:293549757, time:1737391089 (8s ago), events:10, accounts:12
Data: slot:315249991, height:293549758, time:1737391090 (7s ago), events:31, accounts:33
Data: slot:315249992, height:293549759, time:1737391090 (7s ago), events:20, accounts:20
Data: slot:315249993, height:293549760, time:1737391090 (7s ago), events:17, accounts:22
Data: slot:315249994, height:293549761, time:1737391091 (7s ago), events:14, accounts:18
Data: slot:315249995, height:293549762, time:1737391091 (7s ago), events:26, accounts:23
Data: slot:315249996, height:293549763, time:1737391092 (6s ago), events:7, accounts:7
Data: slot:315249997, height:293549764, time:1737391092 (6s ago), events:19, accounts:19
Data: slot:315249998, height:293549765, time:1737391093 (8s ago), events:13, accounts:17
Data: slot:315249999, height:293549766, time:1737391093 (8s ago), events:14, accounts:15
Data: slot:315250000, height:293549767, time:1737391094 (7s ago), events:14, accounts:11
Data: slot:315250001, height:293549768, time:1737391094 (7s ago), events:26, accounts:24
Data: slot:315250002, height:293549769, time:1737391094 (7s ago), events:12, accounts:11
Data: slot:315250003, height:293549770, time:1737391095 (6s ago), events:22, accounts:22
Data: slot:315250004, height:293549771, time:1737391096 (7s ago), events:19, accounts:17
Data: slot:315250005, height:293549772, time:1737391096 (7s ago), events:11, accounts:13
Data: slot:315250006, height:293549773, time:1737391096 (7s ago), events:6, accounts:8
Data: slot:315250007, height:293549774, time:1737391096 (7s ago), events:11, accounts:8
Data: slot:315250008, height:293549775, time:1737391097 (8s ago), events:5, accounts:6
Data: slot:315250009, height:293549776, time:1737391097 (8s ago), events:21, accounts:20
Data: slot:315250010, height:293549777, time:1737391098 (7s ago), events:11, accounts:18
Data: slot:315250011, height:293549778, time:1737391098 (7s ago), events:5, accounts:8
Data: slot:315250012, height:293549779, time:1737391099 (8s ago), events:11, accounts:14
Data: slot:315250013, height:293549780, time:1737391099 (8s ago), events:11, accounts:17
Data: slot:315250014, height:293549781, time:1737391099 (8s ago), events:2, accounts:6
Data: slot:315250015, height:293549782, time:1737391100 (7s ago), events:6, accounts:12
Data: slot:315250016, height:293549783, time:1737391101 (6s ago), events:19, accounts:33
Data: slot:315250017, height:293549784, time:1737391101 (6s ago), events:23, accounts:25
Data: slot:315250018, height:293549785, time:1737391101 (7s ago), events:15, accounts:20
Data: slot:315250019, height:293549786, time:1737391101 (8s ago), events:12, accounts:15
Data: slot:315250020, height:293549787, time:1737391102 (7s ago), events:9, accounts:11
Data: slot:315250021, height:293549788, time:1737391102 (7s ago), events:17, accounts:18
Data: slot:315250022, height:293549789, time:1737391103 (7s ago), events:20, accounts:22
Data: slot:315250023, height:293549790, time:1737391103 (7s ago), events:31, accounts:26
Data: slot:315250024, height:293549791, time:1737391104 (6s ago), events:5, accounts:8
Data: slot:315250025, height:293549792, time:1737391104 (6s ago), events:4, accounts:7
Data: slot:315250026, height:293549793, time:1737391104 (6s ago), events:3, accounts:4
Data: slot:315250027, height:293549794, time:1737391105 (7s ago), events:1, accounts:1
Data: slot:315250028, height:293549795, time:1737391105 (7s ago), events:13, accounts:18
Data: slot:315250029, height:293549796, time:1737391106 (6s ago), events:29, accounts:42
Data: slot:315250030, height:293549797, time:1737391106 (7s ago), events:14, accounts:18
Closed: another stream opened
Connection closed
Reconnect wait...
Connecting to endpoint: wss://orcanauts-b.whirlpool-stream.pleiades.dev slot: Some(315250030)
Data: slot:315250031, height:293549798, time:1737391106 (14s ago), events:11, accounts:12
Data: slot:315250032, height:293549799, time:1737391107 (13s ago), events:18, accounts:20
Data: slot:315250033, height:293549800, time:1737391107 (13s ago), events:16, accounts:16
Data: slot:315250034, height:293549801, time:1737391108 (13s ago), events:20, accounts:20
Data: slot:315250035, height:293549802, time:1737391108 (13s ago), events:16, accounts:19
Data: slot:315250036, height:293549803, time:1737391108 (13s ago), events:22, accounts:28
Data: slot:315250037, height:293549804, time:1737391109 (12s ago), events:24, accounts:26
Data: slot:315250038, height:293549805, time:1737391109 (12s ago), events:33, accounts:24
Data: slot:315250039, height:293549806, time:1737391109 (12s ago), events:28, accounts:45
Data: slot:315250040, height:293549807, time:1737391110 (11s ago), events:27, accounts:33
Data: slot:315250041, height:293549808, time:1737391110 (11s ago), events:14, accounts:14
Data: slot:315250042, height:293549809, time:1737391111 (10s ago), events:20, accounts:25
Data: slot:315250043, height:293549810, time:1737391111 (10s ago), events:17, accounts:22
```
