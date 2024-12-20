mod abi;
mod mocks;

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
    oracle::{self},
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

    let _ = send!(usdc_contract.mint(bob.address(), parse_ether(MINT_AMOUNT).unwrap()));

    let _ = send!(token_contract.mint(alice.address(), parse_ether(MINT_AMOUNT).unwrap()));
    let _ = send!(token_contract.approve(contract_addr, parse_ether(ADMIN_TOTAL_SUPPLY).unwrap()));

    let alice_bal_before = token_contract
        .balanceOf(alice.address())
        .call()
        .await
        .unwrap();

    let _ = send!(contract.initialise(
        alice.address(),
        token_address,
        oracle_address,
        parse_ether(ADMIN_TOTAL_SUPPLY).unwrap(),
        parse_ether("1").unwrap(),
        parse_ether("1").unwrap(),
        vec![usdc_address],
    ))
    .unwrap();

    let alice_bal_after = token_contract
        .balanceOf(alice.address())
        .call()
        .await
        .unwrap();

    assert_eq!(
        alice_bal_before.balance - alice_bal_after.balance,
        parse_ether(ADMIN_TOTAL_SUPPLY).unwrap()
    );

    let token_contract_bal = token_contract
        .balanceOf(contract_addr)
        .call()
        .await
        .unwrap();

    assert_eq!(
        token_contract_bal.balance,
        parse_ether(ADMIN_TOTAL_SUPPLY).unwrap()
    );

    let ITokenSale::isInitialisedReturn { isInitialised } = contract.isInitialised().call().await?;

    assert_eq!(isInitialised, true);

    Ok(())
}

#[e2e::test]
async fn it_can_be_buy_using_usdc(alice: Account, bob: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = ITokenSale::new(contract_addr, &alice.wallet);
    let token_address = erc20::deploy(&alice.wallet).await?;
    let token_contract = ERC20Mock::new(token_address, &alice.wallet);
    let usdc_address = erc20::deploy(&alice.wallet).await?;
    let usdc_contract = ERC20Mock::new(usdc_address, &alice.wallet);
    let oracle_address = oracle::deploy(&alice.wallet).await?;

    let _ = send!(usdc_contract.mint(bob.address(), parse_ether(MINT_AMOUNT).unwrap()));
    let _ = send!(token_contract.mint(alice.address(), parse_ether(MINT_AMOUNT).unwrap()));
    let _ = send!(token_contract.approve(contract_addr, parse_ether(ADMIN_TOTAL_SUPPLY).unwrap()));

    let _ = send!(contract.initialise(
        alice.address(),
        token_address,
        oracle_address,
        parse_ether(ADMIN_TOTAL_SUPPLY).unwrap(),
        parse_ether("1").unwrap(),
        parse_ether("1").unwrap(),
        vec![usdc_address],
    ))
    .unwrap();

    let contract_bob = ITokenSale::new(contract_addr, &bob.wallet);
    let usdc_contract_bob = ERC20Mock::new(usdc_address, &bob.wallet);

    let _ = send!(usdc_contract_bob.approve(contract_addr, parse_ether("10").unwrap())).unwrap();

    let bob_usdc_bal_before = usdc_contract.balanceOf(bob.address()).call().await?;
    let bob_token_bal_before = token_contract.balanceOf(bob.address()).call().await?;

    let _ = send!(contract_bob.buyToken(parse_ether("10").unwrap(), usdc_address, 1)).unwrap();

    let bob_token_bal_after = token_contract.balanceOf(bob.address()).call().await?;
    let bob_usdc_bal_after = usdc_contract.balanceOf(bob.address()).call().await?;

    assert!(bob_token_bal_after.balance > bob_token_bal_before.balance);

    assert!(bob_usdc_bal_after.balance < bob_usdc_bal_before.balance);

    Ok(())
}

#[e2e::test]
async fn admin_can_withdraw(alice: Account, bob: Account) -> Result<()> {
    let contract_addr = alice.as_deployer().deploy().await?.address()?;
    let contract = ITokenSale::new(contract_addr, &alice.wallet);
    let token_address = erc20::deploy(&alice.wallet).await?;
    let token_contract = ERC20Mock::new(token_address, &alice.wallet);
    let usdc_address = erc20::deploy(&alice.wallet).await?;
    let usdc_contract = ERC20Mock::new(usdc_address, &alice.wallet);
    let oracle_address = oracle::deploy(&alice.wallet).await?;

    let _ = send!(usdc_contract.mint(bob.address(), parse_ether(MINT_AMOUNT).unwrap()));
    let _ = send!(token_contract.mint(alice.address(), parse_ether(MINT_AMOUNT).unwrap()));
    let _ = send!(token_contract.approve(contract_addr, parse_ether(ADMIN_TOTAL_SUPPLY).unwrap()));

    let _ = send!(contract.initialise(
        alice.address(),
        token_address,
        oracle_address,
        parse_ether(ADMIN_TOTAL_SUPPLY).unwrap(),
        parse_ether("1").unwrap(),
        parse_ether("1").unwrap(),
        vec![usdc_address],
    ))
    .unwrap();

    let contract_bob = ITokenSale::new(contract_addr, &bob.wallet);
    let usdc_contract_bob = ERC20Mock::new(usdc_address, &bob.wallet);

    let _ = send!(usdc_contract_bob.approve(contract_addr, parse_ether("10").unwrap())).unwrap();

    let _ = send!(contract_bob.buyToken(parse_ether("10").unwrap(), usdc_address, 1)).unwrap();

    let alice_usdc_bal_before = usdc_contract.balanceOf(alice.address()).call().await?;

    let _ = send!(contract.withdraw(usdc_address)).unwrap();

    let alice_usdc_bal_after = usdc_contract.balanceOf(alice.address()).call().await?;

    assert!(alice_usdc_bal_after.balance > alice_usdc_bal_before.balance);

    Ok(())
}
