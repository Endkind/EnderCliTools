use std::io::{self, Read};
use std::process::Command;
use anyhow::{bail, Context, Result};

pub fn ps(all: bool) -> Result<String> {
    let mut args = vec![
        "ps".to_string(),
        "--format".to_string(),
        "{{.ID}};{{.Names}};{{.Image}};{{.Status}};{{.Ports}}".to_string(),
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
