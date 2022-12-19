use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// program input, pass "-" for stdin
    #[clap(long, default_value_t = String::from("-"))]
    pub input: String,
    /// program output, pass "-" for stdout
    #[clap(long, default_value_t = String::from("-"))]
    pub output: String,
}

#[derive(Debug, Default, Clone, Copy, ValueEnum)]
pub enum Encoding {
    #[default]
    NoEncoding,
    Base64,
    Hex,
}

impl Encoding {
    /// input is usually a string
    pub fn decode<T: AsRef<[u8]>>(&self, input: T) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            Self::NoEncoding => input.as_ref().to_vec(),
            Self::Base64 => base64::decode(input)?,
            Self::Hex => hex::decode(input)?,
        })
    }
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Generate keypair
    Generate { bits: usize },
    /// Encrypt message
    Encrypt,
    /// Decrypt message
    Decrypt,
}
