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
    #[argp(option, short='p', arg_name = "PASSWORD")]
    pub password: Option<String>,

    /// Sets the API endpoint
    #[argp(option, short='e', arg_name = "ENDPOINT")]
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
    #[argp(option, short='p', arg_name = "PASSWORD")]
    pub password: Option<String>,

    /// Sets the API endpoint
    #[argp(option, short='e', arg_name = "ENDPOINT")]
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
    pub ids: Vec<String>,

    /// Try the public API endpoint instead of the private one (this will not work for private ships or collections which you do not own)
    #[argp(switch, short='p')]
    pub public: Option<bool>,

    /// Try both the public and private API endpoints, yielding two copies, potnetially
    #[argp(switch, short='b')]
    pub both: Option<bool>,
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
/// Edit a ship or collection by id
pub enum EditOperation
{
    /// Add a key/value pair to a ship or collection
    Add(EditAddOptions),

    /// Remove a key/value pair from a ship or collection
    Remove(EditRemoveOptions),

    /// Set a key/value pair on a ship or collection
    Set(EditSetOptions),
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "add")]
/// Add a key/value pair to a ship or collection
pub struct EditAddOptions
{
    #[argp(positional)]
    /// The key to add
    pub key: String,
    #[argp(positional)]
    /// The value to add
    pub values: Vec<String>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "set")]
/// Add a key/value pair to a ship or collection
pub struct EditSetOptions
{
    #[argp(positional)]
    /// The key to add
    pub key: String,
    #[argp(positional)]
    /// The value to add (multiple values will be joined with a space)
    pub values: Vec<String>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "remove")]
/// Remove a key/value pair from a ship or collection
pub struct EditRemoveOptions
{
    #[argp(positional)]
    /// The key to remove (or remove *from*)
    pub key: String,
    #[argp(positional)]
    /// The value to remove (none will remove all values for the key)
    pub values: Vec<String>,
}

#[derive(FromArgs)]
#[derive(Debug, PartialEq)]
#[argp(subcommand, name = "edit")]
/// Edit a ship or collection by id
pub struct EditOptions
{

    #[argp(switch, short='y')]
    /// Do not prompt for confirmation
    pub yes: Option<bool>,
    #[argp(positional)]
    /// The id of the ship or collection to edit
    pub id: String,

    /// The operation to perform
    #[argp(subcommand)]
    pub operation: EditOperation,
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

    /// Pre-verify a .seria file before uploading (not implemented)
    Verify(VerifyOptions),

    /// Get a ship or collection by id
    Get(GetOptions),

    /// List "ships" "collections" "both" (default=both)
    List(ListOptions),

    /// Fetch a ship or collection by id
    Fetch(FetchOptions),

    /// Edit a ship or collection by id
    Edit(EditOptions),
}
