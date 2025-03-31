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
Example running test:  
```
  $ anchor run test   
  yarn run v1.22.22  
  warning ../../../../../package.json: No license field  
  $ /home/odin/Desktop/software_engineering_laboratory/hangouts/web3_workshop/my-notes-dapp/node_modules/.bin/ts-mocha -p ./tsconfig.json -t 1000000 'tests/**/*.ts'  


    my-notes-app  
  Your transaction signature 2xdmu4cwLw5TFygaa5eD2FReBZax7VzxJLVjNa4dfHt658FReSKXEX6AG61uh51nw7pmoeNviyEEDtDNku7wyNWV  
  Your note {  
    author: PublicKey [PublicKey(5sMdimjMesZ4cscsoLChYc4HGKunVoK6SEyhRBJdUjYQ)] {  
      _bn: <BN: 48551e3af969f20d702d6f79c990019576cb0599be2a8885ff366e6589d8159d>  
    },  
    title: 'Introduction',  
    text: 'This is my life in solitude...',  
    createdAt: <BN: 67ea5f78>,  
    updatedAt: <BN: 67ea5f78>  
  }  
      ✔ Is initialized! (583ms)  


    1 passing (591ms)  

  Done in 2.14s.  
```
