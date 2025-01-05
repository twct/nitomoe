use std::io::Write;

use clap::{Parser, Subcommand};
use rpassword::prompt_password;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Admin {
        #[command(subcommand)]
        admin_command: AdminCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum AdminCommands {
    CreateUser,
}

pub async fn process_args(args: Args) -> std::io::Result<()> {
    match args.command {
        Some(Commands::Admin { admin_command }) => match admin_command {
            AdminCommands::CreateUser => admin_create_user().await,
        },
        None => Ok(()),
    }
}

async fn admin_create_user() -> std::io::Result<()> {
    let username = prompt_input("Username: ")?;
    let password = prompt_password("Password: ")?;

    log::info!("Username = {}, password = <SECURELY_ENTERED>", username);
    log::info!("{}", password);

    Ok(())
}

fn prompt_input<S: Into<String>>(prompt: S) -> std::io::Result<String> {
    print!("{}", prompt.into());
    std::io::stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.trim_end().to_string())
}
