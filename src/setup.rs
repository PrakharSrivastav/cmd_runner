use std::io::stdout;
use structured_logger::{json::new_writer, Builder};

pub fn logger() {
    Builder::new()
        .with_default_writer(new_writer(stdout()))
        //.with_target_writer("application", new_writer(stdout()))
        .init();
}

