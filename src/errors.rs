use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
   error NotOwner();
   error ZeroAddressNotAllowed();
}

#[derive(SolidityError)]
pub enum TokenSaleErrors {
    NotOwner(NotOwner),
    ZeroAddressNotAllowed(ZeroAddressNotAllowed),
}
