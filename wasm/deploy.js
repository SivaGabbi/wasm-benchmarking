var Web3 = require("web3");
var fs = require("fs");
// Connect to our local node
var web3 = new Web3(new Web3.providers.HttpProvider("http://localhost:8545"));
// NOTE: if you run Kovan node there should be an address you've got in the "Option 2: Run Kovan node" step
web3.eth.defaultAccount = "0x1cD92B2DFEf65A55d5F6Cd741Ca6d8c819E46C12";
// read JSON ABI
var abi = JSON.parse(fs.readFileSync("./target/json/TokenInterface.json"));
// convert Wasm binary to hex format
var codeHex = '0x' + fs.readFileSync("./target/pwasm_tutorial_contract.wasm").toString('hex');

var TokenContract = new web3.eth.Contract(abi, { data: codeHex, from: web3.eth.defaultAccount });

var TokenDeployTransaction = TokenContract.deploy({data: codeHex, arguments: [10000000]});

// Will create TokenContract with `totalSupply` = 10000000 and print a result
TokenDeployTransaction.send({gasLimit: '2964078', from: web3.eth.defaultAccount}).then(contract => { console.log("Address of new contract: " + contract.options.address); TokenContract = contract; }).catch(err => console.log(err));