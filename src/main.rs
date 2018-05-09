#![recursion_limit = "1024"]

#[macro_use]
extern crate log;

extern crate env_logger;

extern crate clap;
extern crate dbus;
extern crate hex;
extern crate openssl;
extern crate rand;
extern crate reqwest;

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[cfg(test)]
#[macro_use]
extern crate maplit;

mod args;
mod config_json;
mod configure;
mod deconfigure;
mod errors;
mod fs;
mod generate;
mod logger;
mod os_config;
mod os_config_api;
mod systemd;
mod update;

use args::{get_cli_args, OsConfigSubcommand};
use errors::*;

const SUPERVISOR_SERVICE: &str = "resin-supervisor.service";

fn main() {
    if let Err(ref e) = run() {
        error!("\x1B[1;31mError: {}\x1B[0m", e);

        for inner in e.iter().skip(1) {
            error!("  caused by: {}", inner);
        }

        ::std::process::exit(exit_code(e));
    }
}

fn run() -> Result<()> {
    logger::init_logger();

    let args = get_cli_args();

    match args.subcommand {
        OsConfigSubcommand::GenerateApiKey => generate::generate_api_key(&args),
        OsConfigSubcommand::Update => update::update(&args),
        OsConfigSubcommand::Configure => configure::configure(&args),
        OsConfigSubcommand::Deconfigure => deconfigure::deconfigure(&args),
    }
}
