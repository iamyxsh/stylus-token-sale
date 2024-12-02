#[warn(unused_doc_comments)]
extern crate alloc;

mod constants;
mod errors;
mod interfaces;

use alloy_primitives::U256;
use errors::{TokenSaleErrors, ZeroAddressNotAllowed};
use interfaces::{IOracle, IERC20};
use stylus_sdk::{
    alloy_primitives::Address,
    block, console, contract, msg,
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
    current_price_usd: StorageU256,
    supported_tokens: StorageVec<StorageAddress>,
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

        self.supported_tokens.push(Address::ZERO);
        self.collected_amount.insert(Address::ZERO, U256::ZERO);

        if sale_end < U256::from(block::timestamp()) {
            return Err(TokenSaleErrors::ZeroAddressNotAllowed(
                ZeroAddressNotAllowed {},
            ));
        }

        if supported_tokens.len() > 0 {
            for s_token in supported_tokens {
                self.supported_tokens.push(s_token);
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

    pub fn is_initialised(&self) -> bool {
        self.is_initialised.get()
    }
}
