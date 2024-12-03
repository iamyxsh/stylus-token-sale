# Stylus: Token Sale

A Token Sale contract written in Rust with Stylus SDK to be deployed to Arbitrum Sepolia.

## Getting started

1. You need to start a nitro-dev local node.

```bash
pnpm nitro-node
```

2. Run e2e tests.

```bash
pnpm test:e2e
```

## Basic Idea of the Project.

This token sale was designed to be as close to production as possible. The [Test Token (TST)](https://testnet.routescan.io/address/0x4f5b41d4935969496559230562D8808F242C8dAc/contract/421614/readContract?chainid=421614) is the outgoing token of this contract. In exchange of any of the supported tokens (for now [Test USDC (TUSDC)](https://testnet.routescan.io/address/0x4afeEcEbe5c092Ab2B34390DDee322265b30E89a/contract/421614/code)) based on the price from the [Oracle](https://testnet.routescan.io/address/0x077Da1E3b74FF872E3Ca20452f232D78A092Acf5/contract/421614/code), the user can buy the TST token. The admin can then withdraw the collected amount by calling `fn withdraw()`. The sale will be only live till specified.

These are the protocol contracts -

- [Token Contract](https://sepolia.arbiscan.io/address/0xae0737b533d27742b7bd7d4e0bb3dcad6d78034d)
- [Test Token (TST)](https://testnet.routescan.io/address/0x4f5b41d4935969496559230562D8808F242C8dAc/contract/421614/readContract?chainid=421614)
- [Test USDC (TUSDC)](https://testnet.routescan.io/address/0x4afeEcEbe5c092Ab2B34390DDee322265b30E89a/contract/421614/code)
- [Oracle](https://testnet.routescan.io/address/0x077Da1E3b74FF872E3Ca20452f232D78A092Acf5/contract/421614/code)

## Known Limitations.

1. Front Running

Due to the lack of constructor (technically constructors are supported in stylus, I have not implemented), it is possible that the Token Sale contract can be front run. After the deployment of constructor, any one call the `fn initialise()`. Because of this the pre configured owner should be hardcoded in the contract and should only be allowed to call `fn initialise()`.

2. Contract Size

I encountered an error - "error code -32000: max code size exceeded" due to the size of the contract being more that 24kb. This is the current limit and the most optimized the stylus sdk can do. For this reason, I have commented out some of the basic checks and obvious logic. Current size is around `23.7kb`.
