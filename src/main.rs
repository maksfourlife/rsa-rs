use crate::rsa::{PrivKey, PubKey};
use clap::Parser;
use cli::{Cli, Commands};
use std::io;

pub mod cli;
pub mod rsa;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate { bits } => {
            let (pubkey, privkey) = rsa::gen_keypair(bits);
            println!(
                "===== PUBLIC KEY ====\n{}",
                base64::encode(&bincode::serialize(&pubkey)?)
            );
            println!(
                "==== PRIVATE KEY ====\n{}",
                base64::encode(&bincode::serialize(&privkey)?)
            );
        }
        Commands::Encrypt => {
            let mut pubkey = String::new();
            println!("Pubkey:");
            io::stdin().read_line(&mut pubkey)?;
            let pubkey: PubKey =
                bincode::deserialize(&base64::decode(&pubkey[..pubkey.len() - 1])?)?;

            let mut input = String::new();
            println!("Message:");
            io::stdin().read_line(&mut input)?;

            let cipher_text = base64::encode(pubkey.encrypt(&input[..input.len() - 1]));
            println!("Cypher text: {cipher_text}");
        }
        Commands::Decrypt => {
            let mut privkey = String::new();
            println!("Privkey: ");
            io::stdin().read_line(&mut privkey)?;
            let privkey: PrivKey =
                bincode::deserialize(&base64::decode(&privkey[..privkey.len() - 1])?)?;

            let mut input = String::new();
            println!("Cypher text:");
            io::stdin().read_line(&mut input)?;

            let message =
                String::from_utf8(privkey.decrypt(&base64::decode(&input[..input.len() - 1])?))?;
            println!("Message: {message}");
        }
    }
    Ok(())
}
