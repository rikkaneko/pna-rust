extern crate kvs;

extern crate stderrlog;
extern crate structopt;

use kvs::Result;
use structopt::StructOpt;
use std::net::SocketAddr;

fn main() -> Result<()> {
    let opts = Opts::from_args();
    stderrlog::new()
        .module(module_path!())
        .quiet(opts.quiet)
        .verbosity(2)
        .init()
        .unwrap();
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "kvs-client")]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(
        long = "addr",
        default_value = r#"127.0.0.1:4000"#
    )]
    addr: SocketAddr,
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,
}

#[derive(StructOpt)]
pub enum Command {
    #[structopt(name = "get")]
    Get { key: String },
    #[structopt(name = "set")]
    Set { key: String, value: String },
    #[structopt(name = "rm")]
    Remove { key: String },
}