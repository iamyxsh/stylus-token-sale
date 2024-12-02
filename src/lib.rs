#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

mod constants;
mod errors;
mod interfaces;

use alloc::vec::Vec;
use errors::{TokenSaleErrors, ZeroAddressNotAllowed};
use interfaces::{IOracle, IERC20};
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    block, console, contract, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap, StorageU256},
};

#[storage]
#[entrypoint]
pub struct TokenSale {
    is_initialised: StorageBool,
    admin: StorageAddress,
    token: StorageAddress,
    oracle: StorageAddress,
    total_supply: StorageU256,
    tokens_sold: StorageU256,
    sale_end: StorageU256,
    current_price_usd: StorageU256,
    supported_tokens: StorageMap<Address, StorageBool>,
    collected_amount: StorageMap<Address, StorageU256>,
}

#[public]
impl TokenSale {
    pub fn initialise(
        &mut self,
        admin: Address,
        token: IERC20,
        oracle: IOracle,
        total_supply: U256,
        sale_end: U256,
        initial_price: U256,
        supported_tokens: Vec<Address>,
    ) -> Result<(), TokenSaleErrors> {
        // NOTICE: this check here for the owner is to protect the SC
        // from front-run. Even though stylus technically supports Solidity
        // constructors, I am not implementing that.

        // let owner_address = Address::from_hex(OWNER);
        // if owner_address != msg::sender() {
        //     return Err(TokenSaleErrors::NotOwner(NotOwner {}));
        // }

        if admin.is_zero() || token.is_zero() || oracle.is_zero() {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        self.supported_tokens.insert(Address::ZERO, true);
        self.collected_amount.insert(Address::ZERO, U256::ZERO);

        if sale_end < U256::from(block::timestamp()) {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        if supported_tokens.len() > 0 {
            for s_token in supported_tokens {
                self.supported_tokens.insert(s_token, true);
                self.collected_amount.insert(s_token, U256::ZERO);
            }
        }

        let allowance = token.allowance(&*self, admin, contract::address()).unwrap();
        if allowance < total_supply {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        let ok = token
            .transfer_from(&mut *self, admin, contract::address(), total_supply)
            .unwrap();

        if !ok {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        self.is_initialised.set(true);
        self.admin.set(admin);
        self.token.set(token.address);
        self.tokens_sold.set(U256::ZERO);
        self.oracle.set(oracle.address);
        self.total_supply.set(total_supply);
        self.sale_end.set(sale_end);
        self.current_price_usd.set(initial_price);

        Ok(())
    }

    pub fn buy_token(
        &mut self,
        amount: U256,
        token_in: IERC20,
        price_index: u8,
    ) -> Result<(), TokenSaleErrors> {
        if amount.is_zero() {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        if !self.supported_tokens.get(token_in.address) {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        if self.total_supply.get() < self.tokens_sold.get() + amount {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        let allowance = token_in
            .allowance(&*self, msg::sender(), contract::address())
            .unwrap();
        if allowance < amount {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        let oracle = IOracle::new(self.oracle.get());
        let price = oracle.get_price(&*self, price_index).unwrap();

        let current_price = self.current_price_usd.get();

        let amount_out = TokenSale::calculate_token_out_amount(amount, price, current_price);

        self.tokens_sold.set(self.tokens_sold.get() + amount_out);

        self.current_price_usd.set(self.calculate_price());

        let ok = token_in
            .transfer_from(&mut *self, msg::sender(), contract::address(), amount)
            .unwrap();
        if !ok {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        let token = IERC20::new(self.token.get());
        let ok = token
            .transfer(&mut *self, msg::sender(), amount_out)
            .unwrap();
        if !ok {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        Ok(())
    }

    #[payable]
    pub fn buy(&mut self, amount: U256, price_index: u8) {}

    pub fn is_initialised(&self) -> bool {
        self.is_initialised.get()
    }
}

impl TokenSale {
    fn calculate_token_out_amount(
        amount_in: U256,
        price_in_usd: U256,
        price_out_usd: U256,
    ) -> U256 {
        (amount_in * price_in_usd) / price_out_usd
    }

    fn calculate_price(&self) -> U256 {
        let increments = self.tokens_sold.get() * U256::from(10) / self.total_supply.get();

        let new_price = self.current_price_usd.get()
            + (self.current_price_usd.get() * increments / U256::from(1));

        new_price
    }
}
