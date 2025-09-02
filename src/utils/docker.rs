use std::io::{self, Read};
use std::process::Command;
use anyhow::{bail, Context, Result};
use crate::config::Config;
use crate::config::model::dps::DpsHeader;

pub fn ps(
    all: bool,
    headers: Option<Vec<DpsHeader>>,
    add_headers: Option<Vec<DpsHeader>>,
    last: i32,
    latest: bool,
    no_trunc: bool,
    quiet: bool,
    size: bool,
) -> Result<String> {
    let mut args = vec![
        "ps".into(),
        //"--format".into(), get_headers(headers)?,
        "--last".into(), last.to_string(),
    ];
    if all {
        args.push("--all".to_string());
    }
    if latest {
        args.push("--latest".to_string());
    }
    if no_trunc {
        args.push("--no-trunc".to_string());
    }

    args.push("--format".to_string());
    let base_headers = if quiet {
        Some(vec![DpsHeader::Id])
    } else {
        headers
    };
    let extra_headers = if quiet {
        None
    } else {
        let mut extras = add_headers.unwrap_or_default();

        if size {
            extras.push(DpsHeader::Size);
        };

        if extras.is_empty() {
            None
        } else {
            Some(extras)
        }
    };
    args.push(get_headers(base_headers, extra_headers)?);

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

fn get_headers(headers: Option<Vec<DpsHeader>>, add_headers: Option<Vec<DpsHeader>>) -> Result<String> {
     let mut headers = if let Some(headers) = headers {
        headers
    } else {
        let cfg = Config::load()?;
        cfg.dps.headers
    };

    if let Some(add_headers) = add_headers {
        headers.extend(add_headers);
    }

    let headers_str = headers
        .iter()
        .map(|h| format!("{{{{.{}}}}}", h.display_name()))
        .collect::<Vec<_>>()
        .join(";");

    Ok(headers_str)
}
