use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    // Program ID (replace with your actual program ID)
    let program_id = Pubkey::from_str("D1rbVUwiqNF7vS46g3bs9TCwdkLA4Wa7JcXCFuL9vpWL").unwrap();

    // Connect to the Solana devnet
    // let rpc_url = String::from("http://localhost:8899");
    let rpc_url =
        String::from("https://devnet.helius-rpc.com/?api-key=f621eea1-4f3e-49d7-9b55-cc3b774baa4a");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Generate a new keypair for the payer
    //let payer = Keypair::new();
    // 对于非本地环境， 用一个已知的keypair
    let payer_keypair_path = "/root/.config/solana/id.json";
    let payer = read_keypair_file(payer_keypair_path).expect("Failed to read keypair file");

    // Request airdrop
    /*    let airdrop_amount = 1_000_000_000; // 1 SOL   非本地环境 不要写， 因为没用
        let signature = client
            .request_airdrop(&payer.pubkey(), airdrop_amount)
            .expect("Failed to request airdrop");

        // Wait for airdrop confirmation，  logic is not ok,  it will not stop at   if confirmed , so every round will  define confirmed again, it's bad
        loop {
            let confirmed = client.confirm_transaction(&signature).unwrap();
            if confirmed {
                break;
            }
        }
    */
    // Create the instruction
    let instruction = Instruction::new_with_borsh(
        program_id,
        &(),    // Empty instruction data
        vec![], // No accounts needed
    );

    // Add the instruction to new transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer], client.get_latest_blockhash().unwrap());

    // Send and confirm the transaction
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction Signature: {}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}
