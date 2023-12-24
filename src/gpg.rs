use crate::{error::FaError, store::StoreData};
use dialoguer::Input;
use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct Gpg;

impl Gpg {
    pub fn decrypt(fingerprint: &String, file_path: PathBuf) -> Result<StoreData, FaError> {
        // load file.
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(&file_path)?;
        let mut file_contents = Vec::new();
        file.read_to_end(&mut file_contents)?;

        // decrypt.
        let gpg_decrypt = Command::new("gpg")
            .args(["--quiet", "--local-user", fingerprint, "--decrypt", "--yes"])
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        gpg_decrypt
            .stdin
            .as_ref()
            .ok_or(FaError::UnexpectedNone)?
            .write_all(&file_contents)?;
        let output = gpg_decrypt.wait_with_output()?;
        if output.status.code().ok_or(FaError::UnexpectedNone)? != 0 {
            return Err(FaError::GPGDecryptionError { path: file_path });
        }

        let output = String::from_utf8(output.stdout)?;

        // transform data.
        let store_data: StoreData = match output.is_empty() {
            true => HashMap::new(),
            false => serde_json::from_str::<StoreData>(&output)?,
        };

        Ok(store_data)
    }

    pub fn encrypt(fingerprint: &String, data: &String) -> Result<Vec<u8>, FaError> {
        let gpg_encrypt = Command::new("gpg")
            .args([
                "--quiet",
                "--local-user",
                fingerprint,
                "--default-recipient",
                fingerprint,
                "--sign",
                "--encrypt",
                "--yes",
            ])
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        gpg_encrypt
            .stdin
            .as_ref()
            .ok_or(FaError::UnexpectedNone)?
            .write_all(data.as_bytes())?;

        let output = gpg_encrypt.wait_with_output()?;
        if output.status.code().ok_or(FaError::UnexpectedNone)? != 0 {
            return Err(FaError::GPGEncryptionError);
        }

        Ok(output.stdout)
    }

    pub fn check_if_fingerprint_exists(fingerprint: &String) -> Result<bool, FaError> {
        if fingerprint.len() < 2 {
            return Ok(false);
        }
        let exit_status = Command::new("gpg")
            .args(["-no-tty", "--fingerprint", fingerprint])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;
        Ok(exit_status.success())
    }

    pub fn fingerprint_prompt_until_true_or_term() -> Result<String, FaError> {
        loop {
            let fingerprint: String = Input::new()
                .with_prompt("Enter a GPG Key Fingerprint")
                .interact_text()?;

            if Self::check_if_fingerprint_exists(&fingerprint)? {
                return Ok(fingerprint);
            }
        }
    }
}
