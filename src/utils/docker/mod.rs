pub mod compose;

use crate::config::Config;
use crate::config::model::dps::DpsHeader;
use anyhow::{Context, Result, bail};
use std::io::{self, Read};
use std::process::Command;

#[allow(clippy::too_many_arguments)]
pub fn ps(
    all: bool,
    headers: Option<&[DpsHeader]>,
    add_headers: Option<&[DpsHeader]>,
    last: i32,
    latest: bool,
    no_trunc: bool,
    quiet: bool,
    size: bool,
) -> Result<String> {
    #[rustfmt::skip]
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
    let base_headers: Option<&[DpsHeader]> = if quiet {
        Some(&[DpsHeader::Id])
    } else {
        headers
    };
    let extra_headers = if quiet {
        None
    } else {
        let mut extras = add_headers.unwrap_or_default().to_vec();

        if size {
            extras.push(DpsHeader::Size);
        };

        if extras.is_empty() {
            None
        } else {
            Some(extras)
        }
    };
    args.push(get_headers(base_headers, extra_headers.as_deref())?);

    let attempt = Command::new("docker").args(&args).output();

    match attempt {
        Ok(out) if out.status.success() => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
        _ => {
            if atty::is(atty::Stream::Stdin) {
                bail!(
                    "failed to run `docker {}` and no STDIN provided",
                    args.join(" ")
                )
            }
            let mut buf = String::new();
            io::stdin()
                .read_to_string(&mut buf)
                .context("reading STDIN")?;
            Ok(buf)
        }
    }
}

fn get_headers(headers: Option<&[DpsHeader]>, add_headers: Option<&[DpsHeader]>) -> Result<String> {
    fn build(h: &[DpsHeader], extra: Option<&[DpsHeader]>) -> String {
        h.iter()
            .chain(extra.unwrap_or_default().iter())
            .map(|hdr| format!("{{{{.{}}}}}", hdr.display_name()))
            .collect::<Vec<_>>()
            .join(";")
    }

    let result = if let Some(h) = headers {
        build(h, add_headers)
    } else {
        let cfg = Config::load()?;
        build(&cfg.dps.headers, add_headers)
    };

    Ok(result)
}
