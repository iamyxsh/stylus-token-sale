use alloy_sol_types::sol;
use stylus_sdk::prelude::*;

sol! {
   error NotAdmin();
   error SaleEnded();
   error EndtimeInPast();
}

#[derive(SolidityError)]
pub enum TokenSaleErrors {
    NotAdmin(NotAdmin),
    SaleEnded(SaleEnded),
    EndtimeInPast(EndtimeInPast),
}
