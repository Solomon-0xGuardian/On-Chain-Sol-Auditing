use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey};
use std::str::FromStr;

fn main() {
let rpc_url = "https://api.devnet.solana.com";
let client = RpcClient::new(rpc_url.to_string());

// Replace this with your Solana smart contract (program) ID
let program_id_str = "A8LkBFGFqdiTbk4Z7re7eT6s7174TMUTU2K9srVnpn6E";
let program_id = Pubkey::from_str(program_id_str).expect("Invalid Program ID");

println!("Auditing Solana Smart Contract: {}", program_id);

// Fetch program account data
match client.get_account(&program_id) {
Ok(account_info) => {
println!("Program Account Info: {:?}", account_info);

if account_info.executable {
println!("✅ Program {} is executable.", program_id);
} else {
println!("⚠️ Program {} is NOT executable.", program_id);
}
}
Err(err) => {
println!("❌ Error fetching program data: {}", err);
}
}

// Fetch and print recent transactions involving the contract
match client.get_signatures_for_address(&program_id) {
Ok(transactions) => {
println!("Recent Transactions:");
for tx in transactions.iter().take(5) { // Limit output to 5 transactions
println!("- Signature: {}", tx.signature);
}
}
Err(err) => {
println!("❌ Error fetching transactions: {}", err);
}
}
}
