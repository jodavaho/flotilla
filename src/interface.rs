// Purpose: Defines the CLI interface for Flotilla

use argp::FromArgs;
use std::path::PathBuf;

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(description = "Flotilla - a CLI tool for interacting with the HFOPT API")]
/// Flotilla - a CLI tool for interacting with the HFOPT API
pub struct Cli {

    #[argp(subcommand)]
    /// The subcommand to run
    pub subcommand: SubCommand,

    /// Sets a custom config file
    #[argp(option, short='c', arg_name="FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[argp(switch)]
    pub verbose: Option<bool>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "setup")]
/// Writes a new configuration file
pub struct SetupOptions
{
    /// Sets the username
    #[argp(option, short='u', arg_name = "USERNAME")]
    pub username: Option<String>,

    /// Sets the password
    #[argp(option, arg_name = "PASSWORD")]
    pub password: Option<String>,

    /// Sets the API endpoint
    #[argp(option, arg_name = "ENDPOINT")]
    pub endpoint: Option<String>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "login")]
/// Logs in - you must have a valid username and password, visit the Hfopt website to create an account
pub struct LoginOptions
{
    /// Sets the username
    #[argp(option, short='u', arg_name = "USERNAME")]
    pub username: Option<String>,

    /// Sets the password
    #[argp(option, arg_name = "PASSWORD")]
    pub password: Option<String>,

    /// Sets the API endpoint
    #[argp(option, arg_name = "ENDPOINT")]
    pub endpoint: Option<String>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "logout")]
/// Logs out
pub struct LogoutOptions
{
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "verify")]
/// Pre-verify a .seria file before uploading
pub struct VerifyOptions
{
    /// A .seria file to verify. This will not upload the file, but will ensure that it is valid and ready to upload
    #[argp(positional, arg_name = "FILE")]
    pub file: PathBuf,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "get")]
/// Get a ship or collection by id
pub struct GetOptions
{
    /// The id of the ship or collection to get
    #[argp(positional)]
    pub id: String,
}

fn parse_list_what(s: &str) -> Result<String, String>
{
    match s {
        "ships" => Ok("ships".to_string()),
        "collections" => Ok("collections".to_string()),
        "both" => Ok("both".to_string()),
        _ => Err(String::from("Must be one of ships, collections, or both")),
    }
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "list")]
/// List ships or collections
pub struct ListOptions
{
    /// What to list? Limit to ships or collections, or omit to list both
    #[argp(positional, from_str_fn(parse_list_what))]
    pub what: Option<String>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "fetch")]
/// Fetch a ship or collection by id
pub struct FetchOptions
{
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand)]
pub enum SubCommand
{
    /// Writes a new configuration file
    Setup(SetupOptions),
    /// Logs in - you must have a valid username and password, visit the Hfopt website to create an account
    Login(LoginOptions),
    /// Logs out
    Logout(LogoutOptions),

    /// Pre-verify a .seria file before uploading
    Verify(VerifyOptions),

    /// Get a ship or collection by id
    Get(GetOptions),

    /// List "ships" "collections" "both" (default=both)
    List(ListOptions),

    /// Fetch a ship or collection by id
    Fetch(FetchOptions),
}
