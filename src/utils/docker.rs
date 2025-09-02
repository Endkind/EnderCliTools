use std::io::{self, Read};
use std::process::Command;
use anyhow::{bail, Context, Result};
use crate::config::Config;
use crate::config::model::dps::DpsHeader;

pub fn ps(all: bool, headers: Option<Vec<DpsHeader>>) -> Result<String> {
    let mut args = vec![
        "ps".to_string(),
        "--format".to_string(),
        get_headers(headers)?,
    ];
    if all {
        args.insert(1, "-a".to_string());
    }

    let attempt = Command::new("docker").args(&args).output();

    match attempt {
        Ok(out) if out.status.success() => {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        }
        _ => {
            if atty::is(atty::Stream::Stdin) {
                bail!("failed to run `docker {}` and no STDIN provided", args.join(" "))
            }
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).context("reading STDIN")?;
            Ok(buf)
        }
    }
}

fn get_headers(headers: Option<Vec<DpsHeader>>) -> Result<String> {
    let headers = if let Some(headers) = headers {
        headers
    } else {
        let cfg = Config::load()?;
        cfg.dps.headers
    };

    let headers_str = headers
        .iter()
        .map(|h| format!("{{{{.{}}}}}", h.display_name()))
        .collect::<Vec<_>>()
        .join(";");

    Ok(headers_str)
}
