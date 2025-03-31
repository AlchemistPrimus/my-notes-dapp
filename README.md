# My notes dapp  
Solana rust smart contract with anchor framework. 
A solana based decentralized notebook application using rust anchor framework. 

## Testing the project  
We run the local test validator of solana, deploy the contract and try calling our notebook dapp 
functions.  

We first open the terminal and run the test validator:
`$ solana-test-validator`  
While the initial terminal is still open and running the test validator, open a new terminal and continue 
setting up the contract

### Pointing the solana config to our running local validator  
`$ solana config set --url localhost`  
### Compile the contract  
Program ID is generated and we can view it with `anchor keys list`. 
We can copy this ID and replace it with existing ID in `declare_id` macro in lib.rs and 
Anchor.toml file.  
`$ anchor build`  
### Deploying the contract  
We are deploying the contract to wherever the config is pointing, currently localhost in our case.  
`$ anchor deploy`  
### Testing the contract  
Ensure yarn and ts-mocha is available in your system. The run   
`$ anchor run test`  
Above command makes anchor to use the already set up running test validator.  
