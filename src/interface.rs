// Purpose: Defines the CLI interface for Flotilla

use clap::{Parser,Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub subcommand: SubCommand,

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
    Logout
    {
    },

    /// Pre-verify a .seria file before uploading
    Verify
    {
        /// A .seria file to verify. This will not upload the file, but will ensure that it is valid and ready to upload
        file: PathBuf,
    },

    /// Get a ship or collection by id
    Get
    {
        /// The id of the ship or collection to get
        id: String,
    },

    /// List ships or collections
    #[command(alias = "ls")]
    List
    {
        /// What to list? Limit to ships or collections, or omit to list both
        #[clap(name = "what", value_parser = clap::builder::PossibleValuesParser::new(["ships","collections"]))]
        what: Option<String>,
    },

    /// Fetch latest data from servers (this is often called internally, but you can force it if
    /// you'd like
    Fetch
    {
    },
}
