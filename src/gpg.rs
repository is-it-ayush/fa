use crate::{error::FaError, fa::KEY, store::Credential};
use console::style;
use dialoguer::Input;
use std::{
    io::Write,
    process::{Command, Stdio},
};

pub struct Gpg;

impl Gpg {
    pub fn decrypt(fingerprint: &String, data: Vec<u8>) -> Result<Vec<Credential>, FaError> {
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
            .write_all(&data)?;
        let output = gpg_decrypt.wait_with_output()?;

        if output.status.code().ok_or(FaError::UnexpectedNone)? != 0 {
            return Err(FaError::GPGDecryptionError);
        }

        let output = String::from_utf8(output.stdout)?;

        // transform data.
        let store_data = match output.is_empty() {
            true => Vec::new(),
            false => serde_json::from_str::<Vec<Credential>>(&output)?,
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
        let prompt_str = format!(
            "{} | {}What GPG Key would you like to use to encrypt/decrypt your stores (fingerprint/keyid)?",
            style("[1/3]").bold().dim(),
            KEY
        );
        Input::<String>::new()
            .with_prompt(&prompt_str)
            .validate_with(|input: &String| -> Result<(), FaError> {
                match !Self::check_if_fingerprint_exists(input)? {
                    true => Err(FaError::InvalidFingerprint {
                        fingerprint: input.clone(),
                    }),
                    false => Ok(()),
                }
            })
            .interact_text()
            .map_err(|e| FaError::PromptError { source: e })
    }
}
