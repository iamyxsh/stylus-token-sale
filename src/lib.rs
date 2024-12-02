#[warn(unused_doc_comments)]
extern crate alloc;

mod constants;
mod errors;
mod interfaces;

use std::ops::Add;

use alloy_primitives::{hex::FromHex, U256};
use constants::OWNER;
use errors::{NotOwner, TokenSaleErrors, ZeroAddressNotAllowed};
use interfaces::IERC20;
use stylus_sdk::{
    alloy_primitives::Address,
    console, contract, msg,
    prelude::*,
    storage::{StorageAddress, StorageBool, StorageMap, StorageU256, StorageUint, StorageVec},
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
    supported_tokens: StorageVec<StorageAddress>,
    collected_amount: StorageMap<Address, StorageU256>,
}

#[public]
impl TokenSale {
    pub fn initialise(
        &mut self,
        admin: Address,
        token: IERC20,
        oracle: Address,
        total_supply: U256,
        sale_end: U256,
        supported_tokens: Vec<Address>,
    ) -> Result<(), TokenSaleErrors> {
        /// NOTICE: this check here for the owner is to protect the SC
        /// from front-run. Even though stylus technically supports Solidity
        /// constructors, I am not implementing that.
        // let owner_address = Address::from_hex(OWNER);
        // if owner_address != msg::sender() {
        //     return Err(TokenSaleErrors::NotOwner(NotOwner {}));
        // }
        self.is_initialised.set(true);

        if admin.is_zero() || token.is_zero() || oracle.is_zero() {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        let allowance = token.allowance(&*self, admin, contract::address()).unwrap();
        if allowance < total_supply {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        token
            .transfer_from(&mut *self, admin, contract::address(), total_supply)
            .unwrap();

        Ok(())
    }

    pub fn is_initialised(&self) -> bool {
        self.is_initialised.get()
    }
}
