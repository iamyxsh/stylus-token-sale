#![allow(dead_code)]

use alloy::{primitives::Address, sol};
use e2e::{eyre, Wallet};

sol! {
      enum PRICE_INDEX {
          ARB,
          USDC
      }

      #[sol(rpc, bytecode="6080604052348015600e575f80fd5b506101c38061001c5f395ff3fe608060405234801561000f575f80fd5b5060043610610029575f3560e01c806337f1e7f21461002d575b5f80fd5b61004760048036038101906100429190610104565b61005d565b6040516100549190610147565b60405180910390f35b5f80600181111561007157610070610160565b5b82600181111561008457610083610160565b5b03610099576706f05b59d3b2000090506100d8565b6001808111156100ac576100ab610160565b5b8260018111156100bf576100be610160565b5b036100d457670dbd2fc137a3000090506100d8565b5f90505b919050565b5f80fd5b600281106100ed575f80fd5b50565b5f813590506100fe816100e1565b92915050565b5f60208284031215610119576101186100dd565b5b5f610126848285016100f0565b91505092915050565b5f819050919050565b6101418161012f565b82525050565b5f60208201905061015a5f830184610138565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffdfea2646970667358221220c688b5cec206b54fec11dcf6954a610884f755baa42eee2b495cadcbd076196964736f6c634300081a0033")]
      contract Oracle {
          function getPrice(PRICE_INDEX index) public pure returns (uint256) {
              if (index == PRICE_INDEX.ARB) {
                  return 0.5 ether;
              } else if (index == PRICE_INDEX.USDC) {
                  return 0.99 ether;
              }
              return 0;
          }
      }
}

pub async fn deploy(wallet: &Wallet) -> eyre::Result<Address> {
    let contract = Oracle::deploy(wallet).await?;
    Ok(*contract.address())
}
