mod aoc2022;
mod aoc2023;

use clap::Parser;
use lib::{challenge::ChallengeObject, executor::AocExecutor, inputs::AocInputs};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    worker_threads: Option<usize>,

    #[arg(short, long)]
    year: Option<usize>,

    #[arg(short, long)]
    day: Option<usize>,

    #[arg(long)]
    download: Option<String>,

    #[arg(long)]
    inputs_cache: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut challenges: Vec<ChallengeObject> = vec![aoc2022::challenges(), aoc2023::challenges()]
        .into_iter()
        .flatten()
        .collect();

    if let Some(year) = args.year {
        challenges.retain(|c| c.year == year);
    }

    if let Some(day) = args.day {
        challenges.retain(|c| c.day == day);
    }

    challenges.sort_by_key(|c| c.year * 10 + c.day);

    AocInputs::new(args.inputs_cache.unwrap_or("cache".into()), args.download)?
        .get_inputs(&mut challenges)?;

    println!("Running {} challenges...", challenges.len());

    let mut executor = match args.worker_threads {
        Some(workers) => AocExecutor::with_workers(workers),
        None => AocExecutor::default(),
    };

    let mut results = executor.run_all(challenges);

    results.sort_by_key(|r| r.year * 10 + r.day);

    for result in results {
        println!("[{} day {}] {:?}", result.year, result.day, result.solution);
    }

    Ok(())
}
