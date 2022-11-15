use crate::{
    crydec::EncryptionResult,
    util::{
        math::mod_inverse_mod,
        path::{InputPath, OutputPath, PathValidator},
    },
};
use inquire::{CustomType, Select};
use num_bigint::BigUint;
use std::{fmt, fs, num::NonZeroU64};

mod crydec;
mod util;

#[derive(Debug, Clone, PartialEq, Eq)]
enum WorkMode {
    Encryption,
    Decryption,
}

fn main() -> anyhow::Result<()> {
    loop {
        let mode = Select::new(
            "Select mode:",
            vec![WorkMode::Encryption, WorkMode::Decryption],
        )
        .prompt()?;

        let input_file_path = CustomType::<InputPath>::new("Enter path to the input file:")
            .with_validator(PathValidator)
            .prompt()?;

        let file_data = BigUint::from_bytes_le(fs::read(&input_file_path)?.as_slice());

        let output_file_path = CustomType::<OutputPath>::new("Enter path to the output file:")
            .with_validator(PathValidator)
            .prompt()?;

        match mode {
            WorkMode::Encryption => encryption_mode(file_data, output_file_path)?,
            WorkMode::Decryption => decryption_mode(file_data, output_file_path)?,
        }
    }
}

fn encryption_mode(file_data: BigUint, output_file_path: OutputPath) -> anyhow::Result<()> {
    let EncryptionResult {
        public_key,
        private_key,
        encrypted,
    } = crydec::encrypt(file_data, {
        let key_size = CustomType::<NonZeroU64>::new("Input key base size:").prompt()?;
        println!("Encrypting file...");
        key_size
    })?;

    println!(
        "Public key: {}\nPrivate key: {}\nWriting to file...",
        public_key, private_key
    );

    fs::write(output_file_path, encrypted.to_bytes_le())?;

    println!("Finished writing to file\n\n");

    Ok(())
}

fn decryption_mode(file_data: BigUint, output_file_path: OutputPath) -> anyhow::Result<()> {
    let public_key = CustomType::<BigUint>::new("Enter public key:").prompt()?;
    let private_key = CustomType::<BigUint>::new("Enter private key:").prompt()?;

    println!("Decrypting file...");

    let decrypted = crydec::decrypt(file_data, public_key, private_key);

    println!("Writing to file...");

    fs::write(output_file_path, decrypted.to_bytes_le())?;

    println!("Finished writing to file\n\n");

    Ok(())
}

impl fmt::Display for WorkMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
