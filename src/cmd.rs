use serde::Deserialize;
use std::process::Command;

#[allow(dead_code)]
pub fn run() {
    log::info!("START");

    let cfg_file = std::fs::File::open("config.yaml").expect("could not open file");
    let cfg: Config = serde_yaml::from_reader(cfg_file).expect("could not read value");

    for item in cfg.items {
        log::info!("Running {}", item.name);
        let op: std::process::Output = match Command::new("sh")
            .arg("-c")
            .arg(item.args.as_str())
            .output()
        {
            Ok(op) => op,
            Err(err) => {
                log::error!("failed to run {}", err);
                continue;
            }
        };
        log::info!("results is {}", op.status.to_string());
    }

    log::info!("END");
}

#[derive(Debug, Deserialize)]
struct Config {
    items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
    args: String,
}
