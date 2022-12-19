use crate::rsa::PubKey;
use clap::Parser;
use cli::{Cli, Commands};
use rsa::PrivKey;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{self, Read, Write},
};

pub mod cli;
pub mod math;
pub mod rsa;

fn base64_decode<T: DeserializeOwned>(input: impl AsRef<[u8]>) -> anyhow::Result<T> {
    Ok(bincode::deserialize(&base64::decode(input)?)?)
}

fn base64_encode<T: Serialize + ?Sized>(data: &T) -> anyhow::Result<String> {
    Ok(base64::encode(bincode::serialize(data)?))
}

#[inline]
fn process_generate(bits: usize) -> anyhow::Result<()> {
    let (pubkey, privkey) = rsa::gen_keypair(bits);
    println!(
        r"-----BEGIN RSA PRIVATE KEY-----
{}
-----END RSA PRIVATE KEY-----
-----BEGIN PUBLIC KEY-----
{}
-----END PUBLIC KEY-----
",
        base64_encode(&pubkey)?,
        base64_encode(&privkey)?
    );
    Ok(())
}

/// If path='-' reads line from standard input, else from file
fn read(path: &str) -> io::Result<Vec<u8>> {
    Ok(match path {
        "-" => read_line()?.as_bytes().to_vec(),
        _ => {
            let mut buf = vec![];
            File::open(path)?.read_to_end(&mut buf)?;
            buf
        }
    })
}

fn write(path: &str, data: &[u8]) -> io::Result<()> {
    match path {
        "-" => io::stdout().write_all(data),
        _ => File::create(path)?.write_all(data),
    }
}

/// Reads line from standard input and trims '\n'
#[inline]
fn read_line() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf[..buf.len() - 1].to_string())
}

#[inline]
fn process_encrypt(input: &str, output: &str) -> anyhow::Result<()> {
    let pubkey: PubKey = base64_decode(read_line()?)?;
    // text in file and stdin is not encoded
    let message = read(input)?;
    let cipher = pubkey.encrypt(message);
    match output {
        "-" => println!("{}", base64::encode(cipher)),
        _ => File::create(output)?.write_all(&cipher)?,
    };
    Ok(())
}

#[inline]
fn process_decrypt(input: &str, output: &str) -> anyhow::Result<()> {
    let privkey: PrivKey = base64_decode(read_line()?)?;
    let cipher = match input {
        "-" => base64_decode(read_line()?)?,
        _ => {
            let mut buf = vec![];
            File::open(input)?.read_to_end(&mut buf)?;
            buf
        }
    };
    let message = privkey.decrypt(&cipher);
    write(output, &message)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate { bits } => process_generate(bits)?,
        Commands::Encrypt => process_encrypt(&cli.input, &cli.output)?,
        Commands::Decrypt => process_decrypt(&cli.input, &cli.output)?,
    }
    Ok(())
}
