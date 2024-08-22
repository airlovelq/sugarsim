mod replayer;
mod sender;
mod simulation;
mod step;
mod callbacker;

use log;
use serde_json;
use signal_hook::consts::signal::*;
use signal_hook::iterator::Signals;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::BufReader;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use structopt::StructOpt;
use std::thread;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(short = "c", long = "config")]
    config: String,
}

fn main() {
    // 初始化 simple_logger
    SimpleLogger::new()
        .init()
        .expect("failed to initialize logger");

    let opt = Opt::from_args();
    log::info!("begin simulation task, config file: {}", opt.config);
    let config_file = File::open(opt.config);
    if let Err(e) = config_file {
        log::error!("open config file error: {}", e);
        return 
    }
    log::info!("open config file successfully");
    let reader = BufReader::new(config_file.unwrap());
    let config = serde_json::from_reader(reader);
    if let Err(e) = config {
        log::error!("parse config error: {}", e);
        return 
    }
    log::info!("parse config successfully");
    let config_root = config.unwrap();
    let mut sim_instance = simulation::Simulation::new();

    let res = sim_instance.init(&config_root);
    if res {
        log::info!("init simulator successfully");
    } else {
        log::error!("init simulator failed!");
        return;
    }

    let abort_flag = Arc::clone(&sim_instance.aborted);
    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT, SIGTERM]).unwrap();
        log::info!("set abort signal handler successfully");
        for sig in signals.forever() {
            match sig {
                SIGINT => {
                    log::info!("receive SIGINT signal");
                    abort_flag.store(true, Ordering::SeqCst);
                    break;
                }
                SIGTERM => {
                    log::info!("receive SIGTERM signal");
                    abort_flag.store(true, Ordering::SeqCst);
                    break;
                }
                _ => unreachable!(),
            }
        }
    });

    sim_instance.run();
    // signal_handle.unwrap().join().unwrap();
}
