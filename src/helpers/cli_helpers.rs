use std::fmt::Display;

use colorize::AnsiColor;
use inquire::{Confirm, CustomType, Select};

pub fn prompt_select<T: Display>(
    message: impl Into<String>,
    options: Vec<T>,
    page_size: usize,
) -> Result<T, inquire::InquireError> {
    let message = message.into().green().bold();
    Select::new(message.as_str(), options)
        .with_page_size(page_size)
        .prompt()
}

pub fn prompt_number(message: impl Into<String>) -> Result<usize, inquire::InquireError> {
    let message = message.into().green().bold();
    CustomType::<usize>::new(message.as_str())
        .with_error_message("Please enter a valid non-zero number")
        .with_placeholder("Enter a number")
        .with_parser(&|input: &str| {
            let number = input.parse::<usize>();

            match number {
                Ok(number) if number > 0 => Ok(number),
                _ => Err(()),
            }
        })
        .prompt()
}

pub fn prompt_boolean(message: impl Into<String>) {
    let message = message.into().green().bold();

    let proceed = Confirm::new(message.as_str())
        .with_placeholder("Press 'y' to proceed or 'n' to exit and press Enter")
        .prompt();

    match proceed {
        Ok(true) => {}
        Ok(false) => {
            println!(
                "\n{}",
                "Thank you for using SportRadar CLI! Bye!".yellow().bold()
            );
            std::process::exit(0);
        }
        Err(e) => {
            println!(
                "{}: {:#?}",
                "An error occurred while prompting the user".red(),
                e
            );
            std::process::exit(1);
        }
    }
}
