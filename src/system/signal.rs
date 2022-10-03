use signal_hook::{iterator::Signals, SIGUSR1};
use std::{error::Error, process, thread, time::Duration};

fn main() -> Result<(), Box<dyn Error>> {
    println!("pid: {}", process::id());
}
