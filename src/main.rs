mod config;
mod destination;
mod model;
mod source;

use std::fs::File;
use std::time::Instant;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "config.yml")]
    config: String,
}

fn main() {
    let args = Args::parse();
    let config: config::Config = {
        let config_file = File::open(args.config).unwrap();
        serde_yaml::from_reader(config_file).unwrap()
    };

    let mut sources = vec![];
    for source_config in config.sources {
        let src = source_config.create().unwrap();
        sources.push(src)
    }

    let mut destinations = vec![];
    for destination_config in config.destinations {
        destinations.push(destination_config.create().unwrap());
    }

    for src in sources {
        for dst in &destinations {
            let start = Instant::now();
            let visits = src.get_visits().unwrap();
            dst.push_visits(&visits).unwrap();
            let duration = start.elapsed();

            println!(
                "Copied {} records from {} to {} in {} seconds",
                visits.len(),
                src.name(),
                dst.name(),
                duration.as_secs(),
            )
        }
    }
}
