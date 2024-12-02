use std::str::FromStr;

mod abi;
mod mocks;

use alloy_primitives::{Address, Uint, U256};
// e2e module
use e2e::{
    alloy::{primitives::utils::parse_ether, providers::Provider},
    eyre::Result,
    send, tokio, Account, ReceiptExt,
};

const MINT_AMOUNT: &str = "10000000000";
const ADMIN_TOTAL_SUPPLY: &str = "1000";

use abi::ITokenSale;

use mocks::{
    erc20::{self, ERC20Mock},
    oracle::{self, Oracle},
};

#[e2e::test]
async fn accounts_are_funded(alice: Account) -> Result<()> {
    let balance = alice.wallet.get_balance(alice.address()).await?;
    let expected = parse_ether("100")?;
    assert_eq!(expected, balance);
    Ok(())
}

#[e2e::test]
async fn deploys(alice: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = ITokenSale::new(contract_addr, &alice.wallet);
    let ITokenSale::isInitialisedReturn { isInitialised } = contract.isInitialised().call().await?;

    assert_eq!(isInitialised, false);
    Ok(())
}

#[e2e::test]
async fn it_can_be_initialised(alice: Account, bob: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = ITokenSale::new(contract_addr, &alice.wallet);
    let token_address = erc20::deploy(&alice.wallet).await?;
    let token_contract = ERC20Mock::new(token_address, &alice.wallet);
    let usdc_address = erc20::deploy(&alice.wallet).await?;
    let usdc_contract = ERC20Mock::new(usdc_address, &alice.wallet);
    let oracle_address = oracle::deploy(&alice.wallet).await?;
    let oracle_contract = Oracle::new(oracle_address, &alice.wallet);

    let _ = send!(token_contract.mint(alice.address(), parse_ether(MINT_AMOUNT).unwrap()));
    let _ = send!(token_contract.approve(contract_addr, parse_ether(ADMIN_TOTAL_SUPPLY).unwrap()));

    let bal_before = token_contract
        .balanceOf(alice.address())
        .call()
        .await
        .unwrap();

    let _ = send!(contract.initialise(
        alice.address(),
        token_address,
        oracle_address,
        parse_ether(ADMIN_TOTAL_SUPPLY).unwrap(),
        U256::from(1000000000),
        vec![usdc_address],
    ))
    .unwrap();

    let bal_after = token_contract
        .balanceOf(alice.address())
        .call()
        .await
        .unwrap();

    println!("bal_before {}", bal_before.balance);
    println!("bal_before {}", bal_after.balance);

    let ITokenSale::isInitialisedReturn { isInitialised } = contract.isInitialised().call().await?;

    assert_eq!(isInitialised, true);

    Ok(())
}
