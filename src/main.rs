use structopt::StructOpt;
use std::process::{Command, Stdio};
use std::time::Instant;

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
    let mut ticks = Vec::new();
    for _x in 0..opt.repetitions {
        let elapsed = run_command(&cmd, opt.quiet);
        ticks.push(elapsed);
    }

    ticks.sort();

    let mut sum = 0;
    let mut sum_square = 0;
    for tick in &mut ticks {
        sum += *tick;
        sum_square += *tick * *tick;
    }
    let avg = sum / opt.repetitions as u128;
    // Do I risk loosing some accuracy by casting to f64?
    let std_dev = ((sum_square / opt.repetitions as u128 - avg * avg) as f64).sqrt();

    let p95_index = 0.95 * opt.repetitions as f32 - 1.0;
    let p99_index = 0.99 * opt.repetitions as f32 - 1.0;

    let p95 = if p95_index == p95_index.round() {
        let i1 = ticks[p95_index as usize];
        let i2 = ticks[p95_index as usize + 1];
        (i1 + i2) / 2
    } else {
        ticks[p95_index.ceil() as usize] as u128
    };
    let p99 = if p99_index == p99_index.round() {
        let i1 = ticks[p99_index as usize];
        let i2 = ticks[p99_index as usize + 1];
        (i1 + i2) / 2
    } else {
        ticks[p95_index.ceil() as usize]
    };

    println!("Total time (ms): {}", sum);
    println!("Repetitions: {}", opt.repetitions);
    println!("Average time: {}ms", avg);
    println!("Standard deviation: {}", std_dev);
    println!("p95: {}ms", p95);
    println!("p99: {}ms", p99);
}

fn run_command(cmd: &Vec<String>, quiet: bool) -> u128 {
    let now = Instant::now();
    let _output = if quiet {
        Command::new("sh")
            .arg("-c")
            .args(cmd)
            .stdout(Stdio::null()).stderr(Stdio::null())
            .output().expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .args(cmd)
            .stdout(Stdio::inherit()).stderr(Stdio::inherit())
            .output().expect("failed to execute process")
    };
    return now.elapsed().as_millis()
}
