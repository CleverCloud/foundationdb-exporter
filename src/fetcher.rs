use std::process::Command;

use anyhow::anyhow;
use tracing::error;

use crate::status_models::Status;

pub enum Error {
    Parsing(serde_path_to_error::Error<serde_json::Error>),
    Cmd(anyhow::Error),
}

fn run_command(cmd: &str, args: &[&str]) -> Result<Vec<u8>, anyhow::Error> {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| o.stdout)
        .map_err(|e| {
            error!("Couldn't spawn command '{}': {:#?}", &cmd, e.to_string());
            anyhow!(e.to_string())
        })
}

pub fn fetch_cluster_status(cmd: &str, args: &[&str]) -> Result<Status, Error> {
    let cmd = run_command(cmd, args).map_err(Error::Cmd)?;

    let json_status = &mut serde_json::Deserializer::from_slice(&cmd);
    serde_path_to_error::deserialize(json_status).map_err(|e| {
        error!("Couldn't parse json: {}", e);
        Error::Parsing(e)
    })
}
