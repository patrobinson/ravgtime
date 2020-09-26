use structopt::StructOpt;
use std::process::{Command, Stdio};

#[derive(StructOpt, PartialEq, Debug)]
struct Opt {
    #[structopt(short, required=false, default_value="1")]
    repetitions: u32,
    #[structopt(short)]
    quiet: bool,
    #[structopt(short)]
    histogram: bool,
    #[structopt(subcommand)]
    command: Subcommands,
}

#[derive(StructOpt, PartialEq, Debug)]
enum Subcommands {
    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let opt = Opt::from_args();
    let Subcommands::Other(cmd) = opt.command;
    for _x in 0..opt.repetitions {
        let output = if opt.quiet {
            Command::new("sh")
                .arg("-c")
                .args(&cmd)
                .output().expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .args(&cmd)
                .stdout(Stdio::null()).stderr(Stdio::null())
                .output().expect("failed to execute process")
        };

        println!("{:?}", output)
    }
}
