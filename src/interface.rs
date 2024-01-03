// Purpose: Defines the CLI interface for Flotilla

use clap::{Parser,Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub subcommand: Option<SubCommand>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub verbose: Option<bool>,

}

#[derive(Subcommand)]
pub enum SubCommand
{
    /// Writes a new configuration file
    Setup
    {
        /// Sets the username
        #[arg(short, long, value_name = "USERNAME")]
        username: Option<String>,

        /// Sets the password
        #[arg(short, long, value_name = "PASSWORD")]
        password: Option<String>,

        /// Sets the API endpoint
        #[arg(short, long, value_name = "ENDPOINT")]
        endpoint: Option<String>,
    },
    /// Logs in - you must have a valid username and password, visit the Hfopt website to create an account
    Login
    {
        /// Sets the username
        #[arg(short, long, value_name = "USERNAME")]
        username: Option<String>,

        /// Sets the password
        #[arg(short, long, value_name = "PASSWORD")]
        password: Option<String>,

        /// Sets the API endpoint
        #[arg(short, long, value_name = "ENDPOINT")]
        endpoint: Option<String>,
    },
    /// Logs out
    Logout,
    /// Pre-verify a .seria file before uploading
    Verify
    {
        file: PathBuf,
    }
}

