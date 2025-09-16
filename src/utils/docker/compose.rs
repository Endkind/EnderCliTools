use crate::args::dcps::Status;
use crate::config::Config;
use crate::config::model::dcps::DcpsHeader;
use anyhow::{Context, Result, bail};
use std::io;
use std::process::Command;

#[allow(clippy::too_many_arguments)]
pub fn ps(
    all: bool,
    headers: Option<&[DcpsHeader]>,
    add_headers: Option<&[DcpsHeader]>,
    no_trunc: bool,
    no_orphans: bool,
    quiet: bool,
    services: bool,
    status: Option<&[Status]>,
) -> Result<String> {
    let mut args = vec!["compose".into(), "ps".to_string()];

    if all {
        args.push("--all".into());
    }
    if no_trunc {
        args.push("--no-trunc".into());
    }
    if no_orphans {
        args.push("--orphans=false".into());
    }
    if let Some(status) = status {
        for s in status {
            args.push("--status".into());
            args.push(s.to_string());
        }
    }

    args.push("--format".to_string());
    let base_headers: Option<&[DcpsHeader]> = if quiet {
        Some(&[DcpsHeader::Id])
    } else if services {
        Some(&[DcpsHeader::Service])
    } else {
        headers
    };
    let extra_headers = if quiet | services { None } else { add_headers };
    args.push(get_headers(base_headers, extra_headers)?);

    let attempt = Command::new("docker").args(&args).output();

    match attempt {
        Ok(out) if out.status.success() => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
        _ => {
            if atty::is(atty::Stream::Stdin) {
                bail!(
                    "failed to run `docker compose {}` and so STDIN provided",
                    args.join(" ")
                )
            }
            let mut buf = String::new();
            io::stdin().read_line(&mut buf).context("reading STDIN")?;
            Ok(buf)
        }
    }
}

fn get_headers(
    headers: Option<&[DcpsHeader]>,
    add_headers: Option<&[DcpsHeader]>,
) -> Result<String> {
    fn build(h: &[DcpsHeader], extra: Option<&[DcpsHeader]>) -> String {
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
        build(&cfg.dcps.headers, add_headers)
    };

    Ok(result)
}
