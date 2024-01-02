use std::process::Command;

use anyhow::anyhow;
use tracing::error;

use crate::status_models::Status;

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use crate::fetcher::{fetch_cluster_status, Error};

    use super::run_command;

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn cmd_fails_should_not_panic() {
        let cmd_rs = run_command("exit 1", &[]);
        assert!(cmd_rs.is_err());
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn cmd_fails_on_fetch_should_error_cmd() {
        let cmd_rs = fetch_cluster_status("exit 1", &[]);
        assert!(cmd_rs.is_err());
        assert!(matches!(cmd_rs.err().unwrap(), Error::Cmd { .. }));
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn parsing_fails_on_fetch_should_error_parse() {
        let cmd_rs = fetch_cluster_status("echo", &[]);
        assert!(cmd_rs.is_err());
        assert!(matches!(cmd_rs.err().unwrap(), Error::Parsing { .. }));
    }

    #[cfg(not(target_os = "windows"))]
    #[test]
    fn test_data_parsing_simple_fdb_json() {
        let cmd_rs = fetch_cluster_status("cat", &["tests/data/simple_fdb.json"]);
        assert!(cmd_rs.is_ok());
    }
}
