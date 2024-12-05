use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;
use tokio::time::{sleep, Duration};

const SOLANA_RPC_URL: &str = "https://api.devnet-beta.solana.com"; // Change to your desired Solana RPC URL

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = RpcClient::new(SOLANA_RPC_URL);

    // Replace with the program ID you want to monitor
    let program_id = Pubkey::from_str("YourProgramIdHere")?;

    loop {
        // Fetch accounts associated with the program ID
        let config = RpcProgramAccountsConfig {
            filters: None,
            ..RpcProgramAccountsConfig::default()
        };

        match client.get_program_accounts_with_config(&program_id, config).await {
            Ok(accounts) => {
                for (pubkey, account) in accounts {
                    println!("Account: {}, Data: {:?}", pubkey, account.data);
                    // Here you can implement your logic to analyze the transaction data
                }
            }
            Err(e) => {
                eprintln!("Error fetching accounts: {}", e);
            }
        }

        // Sleep for a while before checking again
        sleep(Duration::from_secs(5)).await;
    }
}
