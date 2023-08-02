use serde::Deserialize;
use std::process::Command;
use std::io::stdout;
use structured_logger::{json::new_writer, Builder};

fn main() {
    setup_logger();
    log::info!("START");

    let cfg_file = std::fs::File::open("config.yaml").expect("could not open file");
    let cfg: Config = serde_yaml::from_reader(cfg_file).expect("could not read value");

    let mut idx = 0;
    loop {
        if idx == cfg.items.len() {
            break;
        }
        log::info!("Running {}", cfg.items[idx].name);
        Command::new("sh")
            .arg("-c")
            .arg(cfg.items[idx].args.as_str())
            .output()
            .map(|op| log::info!("results is {}", op.status.to_string()))
            .map_err(|err| log::error!("failed to run {}", err)).ok();

        idx += 1;
    }

    log::info!("END");
}

fn setup_logger() {
    Builder::new()
        .with_target_writer("application", new_writer(stdout()))
        .init();
}

#[derive(Debug, Deserialize)]
struct Config {
    items: Vec<item>,
}

#[derive(Debug, Deserialize)]
struct item {
    name: String,
    args: String,
}
