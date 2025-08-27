use clap::Args;

#[derive(Args, Debug)]
/// Pretty replacement for `docker ps`
pub struct DpsArgs {
    /// Show all containers (like `docker ps -a`)
    #[arg(short, long)]
    pub all: bool,
}
