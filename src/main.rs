use web3::types::{Address, U256};
use web3::contract::{Contract, Options};

#[tokio::main]
async fn main() -> web3::Result<()> {
    let ws = WebSocket::new("wss://mainnet.infura.io/ws/v3/YOUR_INFURA_PROJECT_ID").await?;
    let web3 = Web3::new(ws);

    let contract_address: Address = "0xYourContractAddress".parse().unwrap();
    let abi = include_bytes!("path/to/your_contract_abi.json");
    let contract = Contract::from_json(web3.eth(), contract_address, abi)?;

    // Example: Call a read-only method
    let result: U256 = contract.query("balanceOf", ("0xYourAddress".parse::<Address>().unwrap(),), None, Options::default(), None).await?;
    println!("Balance: {:?}", result);

    Ok(())
}
