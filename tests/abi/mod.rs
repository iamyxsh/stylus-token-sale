#![allow(dead_code)]
use e2e::alloy::sol;

sol!(
    #[sol(rpc)]
    interface ITokenSale {
        function initialise(address admin, address token, address oracle, uint256 total_supply, uint256 sale_end, address[] memory supported_tokens) external;
        function isInitialised() external view returns (bool isInitialised);

        error NotOwner();
        error ZeroAddressNotAllowed();
}
);
