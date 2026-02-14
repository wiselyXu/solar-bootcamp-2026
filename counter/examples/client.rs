use counter::CounterInstruction;
use solana_client::{rpc_client::RpcClient, rpc_config::CommitmentConfig};
use solana_program::example_mocks::solana_sdk::system_program;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer, read_keypair_file},
    transaction::Transaction,
};
use std::str::FromStr;

#[tokio::main]
async fn main() {
    // Replace with your actual program ID from deployment
    let program_id = Pubkey::from_str("J82USAWey61qAayK4jB9s9Wvj5e8eqyNsTdQwhv3Z2SC")
        .expect("Invalid program ID");

    // Connect to local cluster
    let rpc_url =
        String::from("https://devnet.helius-rpc.com/?api-key=f621eea1-4f3e-49d7-9b55-cc3b774baa4a");
    let client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    let payer_keypair_path = "/root/.config/solana/id.json";
    let payer = read_keypair_file(payer_keypair_path).expect("Failed to read keypair file");

    println!("\nInitializing counter...");
    let counter_keypair = Keypair::new();
    let initial_value = 100u64;

    // Serialize the initialize instruction data
    let instruction_data = borsh::to_vec(&CounterInstruction::InitializeCounter { initial_value })
        .expect("Failed to serialize instruction");

    let initialize_instruction = Instruction::new_with_bytes(
        program_id,
        &instruction_data,
        vec![
            AccountMeta::new(counter_keypair.pubkey(), true),
            AccountMeta::new(payer.pubkey(), true),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );

    let mut transaction =
        Transaction::new_with_payer(&[initialize_instruction], Some(&payer.pubkey()));

    let blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    transaction.sign(&[&payer, &counter_keypair], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Counter initialized!");
            println!("Transaction: {}", signature);
            println!("Counter address: {}", counter_keypair.pubkey());
        }
        Err(err) => {
            eprintln!("Failed to initialize counter: {}", err);
            return;
        }
    }

    println!("\nIncrementing counter...");
    // Serialize the increment instruction data
    let increment_data = borsh::to_vec(&CounterInstruction::IncrementCounter)
        .expect("Failed to serialize instruction");

    let increment_instruction = Instruction::new_with_bytes(
        program_id,
        &increment_data,
        vec![AccountMeta::new(counter_keypair.pubkey(), true)],
    );

    let mut transaction =
        Transaction::new_with_payer(&[increment_instruction], Some(&payer.pubkey()));

    transaction.sign(&[&payer, &counter_keypair], blockhash);

    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Counter incremented!");
            println!("Transaction: {}", signature);
        }
        Err(err) => {
            eprintln!("Failed to increment counter: {}", err);
        }
    }
}
