var TobalabaTestCoin = artifacts.require("./TobalabaTestCoin.sol");

module.exports = function(deployer) {
  deployer.deploy(TobalabaTestCoin);
};
