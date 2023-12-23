use crate::error::FaError;
use dialoguer::Input;
use std::process::{Command, Stdio};

pub struct Gpg;

impl Gpg {
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
