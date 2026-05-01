use inquire::{Password, Text, validator::Validation};
use regex::Regex;

fn main() {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();

    let email = Text::new("What is your email?")
        .with_validator(move |input: &str| {
            if email_regex.is_match(input) {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid("Invalid email format".into()))
            }
        })
        .prompt();

    if email.is_err() {
        return;
    }

    let password = Password::new("Enter your password")
        .without_confirmation()
        .with_validator(|input: &str| {
            let has_letter = input.chars().any(|c| c.is_alphabetic());
            let has_number = input.chars().any(|c| c.is_numeric());
            if input.len() >= 8 && has_letter && has_number {
                Ok(Validation::Valid)
            } else {
                Ok(Validation::Invalid(
                    "Must be 8+ chars with letters and numbers".into(),
                ))
            }
        })
        .prompt();

    match password {
        Ok(_) => println!("Registration successful!"),
        Err(_) => println!("Cancelled."),
    }
}
