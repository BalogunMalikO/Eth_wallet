use anyhow::Result;
mod eth_walet;
mod utils;
use std::env;
use std::io;
use std::str::FromStr;
use web3::types::Address;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    // let (secret_key, pub_key) = eth_walet::generate_keypair();

    // println!("secret key: {}", secret_key.to_string());
    // println!("public key: {}", pub_key.to_string());

    // let pub_addy = eth_walet::public_key_addy(&pub_key);
    // println!("Public address: {:?}", pub_addy);

    // let crypto_wallet = eth_walet::Wallet::new(&secret_key, &pub_key);
    // println!("crypto_wallet {:?}", &crypto_wallet);

    let wallet_file_path ="crypto_wallet.json";

    // crypto_wallet.save_to_file(wallet_file_path)?;

    let loaded_wallet = eth_walet::Wallet::Read_from_file(wallet_file_path)?;
    println!("loaded_wallet: {:?}", loaded_wallet);

    let endpoint = env::var("INFURA_GOEARLI_WS")?;
    let web3_con = eth_walet::establish_web3_connection(&endpoint).await?;

    let block_number = web3_con.eth().block_number().await?;
    println!("block number: {}", &block_number);

    let balance = loaded_wallet.get_balance_in_eth(&web3_con).await?;
    println!("Balance: {}", &balance);

    println!("Please enter to send to: ");
    let mut addy = String::new();
    io::stdin().read_line(&mut addy).expect("value address");
    
   
    println!("Please enter Value: ");
    let mut value_to_send = String::new();
    io::stdin().read_line(&mut value_to_send).expect("please enter a value");
    
     let value: f64 = value_to_send.trim().parse().unwrap_or_default();
        
          
       
    

   let transaction = eth_walet::create_eth_transaction(
        Address::from_str(&addy)?,
        value.clone(),
    );
    let transact_hash =
        eth_walet::sign_and_send(&web3_con, transaction, loaded_wallet.get_secret_key()?).await?;
    println!("transaction hash: {:?}", transact_hash);


    Ok(())
    
 
}

