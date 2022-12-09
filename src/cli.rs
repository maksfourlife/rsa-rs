use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
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
