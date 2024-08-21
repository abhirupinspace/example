// use clap::{Arg, Command};
// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{
//     pubkey::Pubkey,
//     signature::{Keypair, Signature, Signer},
//     system_instruction,
//     transaction::Transaction,
// };
// use std::{error::Error, str::FromStr};
// // use bs58;  // Add this line to import the bs58 crate
// use solana_sdk::bs58;

// const SOLANA_URL: &str = "http://localhost:8899";

// // Function to create a keypair from a base58 string
// fn keypair_from_base58(base58_string: &str) -> Result<Keypair, Box<dyn Error>> {
//     let keypair_bytes = bs58::decode(base58_string).into_vec()?;
//     let keypair = Keypair::from_bytes(&keypair_bytes)?;
//     Ok(keypair)
// }

// // Main function for the CLI program
// fn main() -> Result<(), Box<dyn Error>> {
//     // Define the CLI arguments
//     let matches = Command::new("Solana CLI")
//         .version("1.0")
//         .author("Rome Example")
//         .about("Sends an atomic transaction to two separate rollups")
//         .arg(
//             Arg::new("to1")
//                 .short('t')
//                 .long("to1")
//                 .value_name("TO_ADDRESS_1")
//                 .help("First rollup recipient address")
//                 .required(true),
//         )
//         .arg(
//             Arg::new("to2")
//                 .short('u')
//                 .long("to2")
//                 .value_name("TO_ADDRESS_2")
//                 .help("Second rollup recipient address")
//                 .required(true),
//         )
//         .arg(
//             Arg::new("from")
//                 .short('f')
//                 .long("from")
//                 .value_name("FROM_BASE58_KEYPAIR")
//                 .help("Base58 encoded sender keypair")
//                 .required(true),
//         )
//         .arg(
//             Arg::new("amount")
//                 .short('a')
//                 .long("amount")
//                 .value_name("AMOUNT")
//                 .help("Amount to send")
//                 .required(true),
//         )
//         .get_matches();

//     // Get the arguments from the CLI
//     let to_address1 = matches.get_one::<String>("to1").unwrap();
//     let to_address2 = matches.get_one::<String>("to2").unwrap();
//     let from_base58_keypair = matches.get_one::<String>("from").unwrap();
//     let amount: u64 = matches.get_one::<String>("amount").unwrap().parse()?;

//     // Load the keypair for signing the transaction
//     let sol_keypair = keypair_from_base58(from_base58_keypair)?;

//     // Initialize Solana RPC client
//     let rpc_client = RpcClient::new(SOLANA_URL.to_string());

//     // Send the atomic transaction to two separate rollups
//     send_atomic_transaction(&rpc_client, &sol_keypair, to_address1, to_address2, amount)?;

//     Ok(())
// }

// // Function to send the atomic transaction
// fn send_atomic_transaction(
//     rpc_client: &RpcClient,
//     sol_keypair: &Keypair,
//     to_address1: &str,
//     to_address2: &str,
//     amount: u64,
// ) -> Result<Signature, Box<dyn Error>> {
//     let from_pubkey = sol_keypair.pubkey();
//     let to_pubkey1 = Pubkey::from_str(to_address1)?;
//     let to_pubkey2 = Pubkey::from_str(to_address2)?;

//     // Create transfer instructions for the two rollups
//     let instruction1 = system_instruction::transfer(&from_pubkey, &to_pubkey1, amount);
//     let instruction2 = system_instruction::transfer(&from_pubkey, &to_pubkey2, amount);

//     // Create a transaction containing both instructions
//     let transaction = Transaction::new_signed_with_payer(
//         &[instruction1, instruction2],
//         Some(&from_pubkey),
//         &[sol_keypair],
//         rpc_client.get_latest_blockhash()?,
//     );

//     // Send the transaction
//     let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

//     println!("Transaction sent with signature: {}", signature);

//     Ok(signature)
// }


use clap::{Arg, Command};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
    bs58
};
use std::{error::Error, str::FromStr};

const SOLANA_URL: &str = "http://localhost:8899";

// Function to load a keypair from a Base58 string
fn keypair_from_base58(base58_string: &str) -> Result<Keypair, Box<dyn Error>> {
    let keypair_bytes = bs58::decode(base58_string).into_vec()?;
    
    println!("Keypair bytes length: {}", keypair_bytes.len());

    // Ensure the keypair bytes are 64 bytes long
    if keypair_bytes.len() != 64 {
        return Err("Keypair byte array length is not 64 bytes".into());
    }

    // Convert the bytes to a Keypair
    let keypair = Keypair::from_bytes(&keypair_bytes)?;
    Ok(keypair)
}

// Main function for the CLI program
fn main() -> Result<(), Box<dyn Error>> {
    // Define the CLI arguments
    let matches = Command::new("Solana CLI")
        .version("1.0")
        .author("Rome Example")
        .about("Sends an atomic transaction to two separate rollups")
        .arg(
            Arg::new("to1")
                .short('t')
                .long("to1")
                .value_name("TO_ADDRESS_1")
                .help("First rollup recipient address")
                .required(true),
        )
        .arg(
            Arg::new("to2")
                .short('u')
                .long("to2")
                .value_name("TO_ADDRESS_2")
                .help("Second rollup recipient address")
                .required(true),
        )
        .arg(
            Arg::new("from")
                .short('f')
                .long("from")
                .value_name("FROM_BASE58_KEYPAIR")
                .help("Base58 encoded sender keypair")
                .required(true),
        )
        .arg(
            Arg::new("amount")
                .short('a')
                .long("amount")
                .value_name("AMOUNT")
                .help("Amount to send")
                .required(true),
        )
        .get_matches();

    // Get the arguments from the CLI
    let to_address1 = matches.get_one::<String>("to1").unwrap();
    let to_address2 = matches.get_one::<String>("to2").unwrap();
    let from_base58_keypair = matches.get_one::<String>("from").unwrap();
    let amount: u64 = matches.get_one::<String>("amount").unwrap().parse()?;

    // Load the keypair for signing the transaction
    let sol_keypair = keypair_from_base58(from_base58_keypair)?;

    // Initialize Solana RPC client
    let rpc_client = RpcClient::new(SOLANA_URL.to_string());

    // Send the atomic transaction to two separate rollups
    send_atomic_transaction(&rpc_client, &sol_keypair, to_address1, to_address2, amount)?;

    Ok(())
}

// Function to send the atomic transaction
fn send_atomic_transaction(
    rpc_client: &RpcClient,
    sol_keypair: &Keypair,
    to_address1: &str,
    to_address2: &str,
    amount: u64,
) -> Result<(), Box<dyn Error>> {
    let from_pubkey = sol_keypair.pubkey();
    let to_pubkey1 = Pubkey::from_str(to_address1)?;
    let to_pubkey2 = Pubkey::from_str(to_address2)?;

    // Create transfer instructions for the two rollups
    let instruction1 = system_instruction::transfer(&from_pubkey, &to_pubkey1, amount);
    let instruction2 = system_instruction::transfer(&from_pubkey, &to_pubkey2, amount);

    // Create a transaction containing both instructions
    let transaction = Transaction::new_signed_with_payer(
        &[instruction1, instruction2],
        Some(&from_pubkey),
        &[sol_keypair],
        rpc_client.get_latest_blockhash()?,
    );

    // Send the transaction
    let signature = rpc_client.send_and_confirm_transaction(&transaction)?;

    println!("Transaction sent with signature: {}", signature);

    Ok(())
}
