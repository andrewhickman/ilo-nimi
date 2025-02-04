use std::process::exit;

use clap::Parser;
use ilo_nimi::NameGenerator;
use rand::prelude::*;
use sha2::{Digest, Sha256};

/// Generates random names.
#[derive(Parser)]
struct Args {
    // Minimum length for name.
    #[clap(long, default_value = "1", alias = "min")]
    min_length: u32,
    // Maximum length for name.
    #[clap(long, alias = "max")]
    max_length: Option<u32>,
    // Number of names to generate.
    #[clap(short = 'n', long, default_value = "1")]
    count: usize,
    // Output names in title-case.
    #[clap(long, alias = "title")]
    title_case: bool,
    // Seed for name generation
    #[clap(long)]
    seed: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args
        .max_length
        .is_some_and(|max| max == 0 || max < args.min_length)
    {
        eprintln!("invalid max value");
        exit(1);
    }

    let mut rng = if let Some(seed) = args.seed {
        SmallRng::from_seed(Sha256::digest(seed).into())
    } else {
        SmallRng::from_os_rng()
    };

    let generator = NameGenerator::new(args.min_length, args.max_length);

    for _ in 0..args.count {
        let name = generator.generate(&mut rng, args.title_case);
        println!("{name}");
    }
}
