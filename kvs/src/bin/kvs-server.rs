extern crate kvs;
extern crate structopt;

use kvs::cmdline::*;
use kvs::Result;
use std::str;
use structopt::StructOpt;

fn main() -> Result<()> {
    Opts::from_args();
    Ok(())
}

#[derive(StructOpt)]
#[structopt(name = "kvs-server")]
struct Opts {
    #[structopt(subcommand)]
    cmd: Command,
    #[structopt(
        long = "addr",
        default_value = r#"("127.0.0.1", 4000)"#,
        parse(try_from_str = "parse_addr")
    )]
    addr: (String, u32),
    #[structopt(raw(possible_values = r#"&["kvs", "sled"]"#))]
    engine_name: String,
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
