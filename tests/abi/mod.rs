#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
    #[sol(rpc)]
    interface ITokenSale {
    function initialise(address admin, address token, address oracle, uint256 total_supply, uint256 sale_end, uint256 initial_price, address[] memory supported_tokens) external;

    function buyToken(uint256 amount, address token_in, uint8 price_index) external;

    function buy(uint256 amount, uint8 price_index) external payable;

    function isInitialised() external view returns (bool isInitialised);

    error NotOwner();

    error ZeroAddressNotAllowed();
}
);
