use crate::{
    error::{FaError, FaErrorCodes},
    store::StoreData,
};
use dialoguer::Input;
use std::{
    collections::HashMap,
    io::Write,
    process::{Command, Stdio},
};

pub struct Gpg;

impl Gpg {
    pub fn decrypt(fingerprint: &String, data: Vec<u8>) -> Result<StoreData, FaError> {
        let gpg_decrypt = Command::new("gpg")
            .args([
                "--quiet",
                "--local-user",
                fingerprint,
                "--decrypt",
                "--yes",
            ])
            .stderr(Stdio::null())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        gpg_decrypt
            .stdin
            .as_ref()
            .ok_or(FaError::new(
                FaErrorCodes::Generic,
                "Could not take a ref to stdin during decryption.",
            ))?
            .write_all(&data)?;

        let decrypted_buffer = gpg_decrypt.wait_with_output()?.stdout;
        let file_content = String::from_utf8(decrypted_buffer)?;

        // transform data.
        let store_data: StoreData = match file_content.is_empty() {
            true => HashMap::new(),
            false => serde_json::from_str::<StoreData>(&file_content)?,
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
            .ok_or(FaError::new(
                FaErrorCodes::Generic,
                "Could not take a ref to stdin during encryption.",
            ))?
            .write_all(data.as_bytes())?;

        let encrypted_data = gpg_encrypt.wait_with_output()?.stdout;
        Ok(encrypted_data)
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
