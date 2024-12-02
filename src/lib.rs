#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

mod constants;
mod errors;
mod interfaces;

use alloc::vec::Vec;
use errors::{EndtimeInPast, NotAdmin, SaleEnded, TokenSaleErrors};
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
    collected_amount: StorageMap<Address, StorageU256>,
}

#[public]
impl TokenSale {
    pub fn initialise(
        &mut self,
        admin: Address,
        token: Address,
        oracle: Address,
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

        // NOTICE: commented out due to "max code size exceeded" reason

        // if admin.is_zero() || token.is_zero() || oracle.is_zero() {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        // NOTICE: commented out due to "max code size exceeded" reason
        // Also, Address Zero notifies Native Gas Currency which can be used
        // to buy tokens. Commented out as I will not be able to implement that
        // due to the contract size issue.

        // self.collected_amount.insert(Address::ZERO, U256::ZERO);

        if sale_end < U256::from(block::timestamp()) {
            return Err(TokenSaleErrors::EndtimeInPast(EndtimeInPast {}));
        }

        if supported_tokens.len() > 0 {
            for s_token in supported_tokens {
                self.collected_amount.insert(s_token, U256::ZERO);
            }
        }

        // NOTICE: commented out due to "max code size exceeded" reason

        // let allowance = token.allowance(&*self, admin, contract::address()).unwrap();
        // if allowance < total_supply {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        self.transfer_token_from(token, total_supply, admin, contract::address());

        // NOTICE: commented out due to "max code size exceeded" reason

        // if !ok {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        self.is_initialised.set(true);
        self.admin.set(admin);
        self.token.set(token);
        self.tokens_sold.set(U256::ZERO);
        self.oracle.set(oracle);
        self.total_supply.set(total_supply);
        self.sale_end.set(sale_end);
        self.current_price_usd.set(initial_price);

        Ok(())
    }

    pub fn buy_token(
        &mut self,
        amount: U256,
        token_in: Address,
        price_index: u8,
    ) -> Result<(), TokenSaleErrors> {
        if self.sale_end.get() < U256::from(block::timestamp()) {
            return Err(TokenSaleErrors::SaleEnded(SaleEnded {}));
        }
        // NOTICE: commented out due to "max code size exceeded" reason

        // if amount.is_zero() {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        // NOTICE: commented out due to "max code size exceeded" reason

        // if !self.supported_tokens.get(token_in.address) {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        // NOTICE: commented out due to "max code size exceeded" reason

        // if self.total_supply.get() < self.tokens_sold.get() + amount {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        // NOTICE: commented out due to "max code size exceeded" reason

        // let allowance = token_in
        //     .allowance(&*self, msg::sender(), contract::address())
        //     .unwrap();
        // if allowance < amount {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }
        let amount_out = self.calculate_amount_out(amount, price_index);

        self.tokens_sold.set(self.tokens_sold.get() + amount_out);

        self.current_price_usd.set(self.calculate_price());

        self.transfer_token_from(token_in, amount, msg::sender(), contract::address());

        // NOTICE: commented out due to "max code size exceeded" reason

        // if !ok {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }

        self.transfer_token(self.token.get(), amount_out, msg::sender());

        self.set_collected_amount(token_in, self.collected_amount.get(token_in) + amount);

        Ok(())
    }

    pub fn withdraw(&mut self, token_addr: Address) -> Result<(), TokenSaleErrors> {
        if msg::sender() != self.admin.get() {
            return Err(TokenSaleErrors::NotAdmin(NotAdmin {}));
        }

        self.transfer_token(
            token_addr,
            self.collected_amount.get(token_addr),
            msg::sender(),
        );

        self.set_collected_amount(token_addr, U256::from(0));

        Ok(())
    }

    pub fn is_initialised(&self) -> bool {
        self.is_initialised.get()
    }

    pub fn calculate_amount_out(&self, amount: U256, price_index: u8) -> U256 {
        let price = self.get_price(price_index);

        let current_price = self.current_price_usd.get();

        (amount * price) / current_price
    }
}

impl TokenSale {
    fn calculate_price(&self) -> U256 {
        let increments = self.tokens_sold.get() * U256::from(10) / self.total_supply.get();

        let new_price = self.current_price_usd.get()
            + (self.current_price_usd.get() * increments / U256::from(1));

        new_price
    }

    fn get_price(&self, price_index: u8) -> U256 {
        let oracle = IOracle::new(self.oracle.get());
        oracle.get_price(&*self, price_index).unwrap()
    }

    fn transfer_token(&mut self, token_addr: Address, amount: U256, to: Address) {
        let token = IERC20::new(token_addr);
        let _ = token.transfer(&mut *self, to, amount).unwrap();

        // NOTICE: commented out due to "max code size exceeded" reason

        // if !ok {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }
    }

    fn transfer_token_from(
        &mut self,
        token_addr: Address,
        amount: U256,
        from: Address,
        to: Address,
    ) {
        let token = IERC20::new(token_addr);
        let _ = token.transfer_from(&mut *self, from, to, amount).unwrap();

        // NOTICE: commented out due to "max code size exceeded" reason

        // if !ok {
        //     return Err(TokenSaleErrors::ZeroAddressNotAllowed(
        //         ZeroAddressNotAllowed {},
        //     ));
        // }
    }

    fn set_collected_amount(&mut self, token_addr: Address, new_amount: U256) {
        let mut amount_setter = self.collected_amount.setter(token_addr);
        amount_setter.set(new_amount);
    }
}
