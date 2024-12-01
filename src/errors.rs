use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
   #[derive(Debug)]
   error NotOwner();

   #[derive(Debug)]
   error ZeroAddressNotAllowed();
}

#[derive(SolidityError, Debug)]
pub enum TokenSaleErrors {
    NotOwner(NotOwner),
    ZeroAddressNotAllowed(ZeroAddressNotAllowed),
}
