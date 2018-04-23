use errors::*;

use args::Args;

use config_json::read_config_json;
use provision::reconfigure;

pub fn update(args: &Args) -> Result<()> {
    let config_json = read_config_json(&args.config_json_path)?;

    reconfigure(args, config_json, false)
}
