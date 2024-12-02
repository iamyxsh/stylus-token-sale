use stylus_sdk::prelude::sol_interface;

sol_interface! {
  interface IERC20 {
    function transfer(address to, uint256 value) external returns (bool);
    function allowance(address owner, address spender) external view returns (uint256);
    function transferFrom(address from, address to, uint256 value) external returns (bool);
  }
}

sol_interface! {
  interface IOracle {
    function getPrice(uint8 index) external pure returns (uint256) {}
  }
}
