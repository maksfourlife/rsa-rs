use crate::rsa::{PrivKey, PubKey};
use clap::Parser;
use cli::{Cli, Commands};
use serde::{de::DeserializeOwned, Serialize};
use std::io;

pub mod cli;
pub mod rsa;

fn base64_decode<T: DeserializeOwned>(input: impl AsRef<[u8]>) -> anyhow::Result<T> {
    Ok(bincode::deserialize(&base64::decode(input)?)?)
}

fn base64_encode<T: Serialize + ?Sized>(data: &T) -> anyhow::Result<String> {
    Ok(base64::encode(bincode::serialize(data)?))
}

fn read_line(message: impl AsRef<str>) -> io::Result<String> {
    println!("{}", message.as_ref());
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate { bits } => {
            let (pubkey, privkey) = rsa::gen_keypair(bits);
            println!("Pubkey:\n{}", base64_encode(&pubkey)?);
            println!("Privkey:\n{}", base64_encode(&privkey)?);
        }
        Commands::Encrypt => {
            let pubkey: PubKey = base64_decode(read_line("Pubkey: ")?.trim_end())?;
            let message = read_line("Message: ")?;
            let cypher_text = base64::encode(pubkey.encrypt(&message));
            println!("Cypher text: {cypher_text}");
        }
        Commands::Decrypt => {
            let privkey: PrivKey = base64_decode(read_line("Privkey: ")?.trim_end())?;
            let cypher_text = base64::decode(read_line("Cypher text:")?.trim_end())?;
            let message = String::from_utf8(privkey.decrypt(&cypher_text))?;
            println!("Message: {message}");
        }
    }
    Ok(())
}
