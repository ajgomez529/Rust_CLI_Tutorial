use structopt::StructOpt;
// use std::io::{BufReader, BufRead, Error};
// use std::fs::File;

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let _args = Cli::from_args();
    let content = std::fs::read_to_string(&_args.path)?;
    grrs::find_matches(&content, &_args.pattern, &mut std::io::stdout());
    Ok(())
}
