use thiserror::Error;
use std::fmt;
use validator::Validate;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, PartialEq)]
pub enum EmailError {
    #[error("Invalid email format")]
    InvalidEmail,
}


#[derive(Validate, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(transparent)]
pub struct Email{
    #[validate(email)]
    value: String
}

impl Email { 
    pub fn new(input: impl Into<String>) -> Result<Email, EmailError> { 
        let email = Email{value: input.into()};
        email.validate()
            .map(|_| email)
            .map_err(|_| EmailError::InvalidEmail)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }
}

impl fmt::Debug for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Email")
            .field(&format_args!("{}",mask_email(&self.value)))
            .finish()
    }
}

/// Masks an email address by replacing all characters except the first
/// with asterisks (*). If the email address does not contain an '@'
/// character, it is returned unchanged.
fn mask_email(email: &str) -> String {
    match email.split_once('@') {
        Some((local, domain)) => {
            let visible = local.chars().next().unwrap_or('*');
            format!("{}***@{}", visible, domain)
        },
        None => email.to_string(),
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! new_email_test_cases {
        (
            $(
                ($test_name: ident, $email: expr, $should_be_error: expr)
            ),*
        ) => {
            $(
                #[test]
                fn $test_name() {
                    assert_eq!(
                        Email::new($email).is_err(),
                        $should_be_error
                    )
                }
            )*
        };
    }

    new_email_test_cases!{
        (new_valid_email, "harun@asolole.com", false),
        (new_empty_email, "", true),
        (new_invalid_email, "harun.asolole.com", true)
    }

    
    macro_rules! debug_email_test_cases {
        (
            $(
                ($test_name: ident, $email: expr, $expected: expr)
            ),*
        ) => {
            $(
                #[test]
                fn $test_name() {
                    assert_eq!(
                        format!("{:?}", Email{value: $email.into()}),
                        $expected
                    )
                }
            )*
        };
    }

    debug_email_test_cases!{
        (debug_valid_email, "harun@asolole.com", "Email(h***@asolole.com)"),
        (debug_empty_email, "", "Email()"),
        (debug_invalid_email, "harun.asolole.com", "Email(harun.asolole.com)")
    }
}