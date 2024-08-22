use crate::sender::Sender;
use crate::step::Step;
use log;
use serde_json::Value;
use std::{any::Any, thread, time};

pub trait Replayer {
    fn init(&mut self, config: &Value) -> bool;
    fn step(&mut self, step: &Step, backdata: &Option<Box<dyn Any>>) -> Vec<Sender>;

    fn finished(&self) -> bool;
}

struct MockReplayer {
    value: i32,
    finished: bool,
}

impl Replayer for MockReplayer {
    fn init(&mut self, config: &Value) -> bool {
        self.value = 0;
        self.finished = false;
        log::info!("replayer config is {}", config.to_string());
        true
    }
    fn step(&mut self, step: &Step, backdata: &Option<Box<dyn Any>>) -> Vec<Sender> {
        // self.value += 1;
        let mut vec: Vec<Sender> = Vec::new();
        if self.value > 120 {
            self.finished = true;
            return vec;
        }
        if let Some(backdata_value) = backdata.as_ref() {
            if let Some(int_value) = backdata_value.downcast_ref::<i32>() {
                self.value += int_value;
            }
        }
        let value = self.value;
        let timestamp = step.cur_time();

        vec.push(Sender {
            timestamp: 0,
            executor: Box::new(move || {
                log::info!("current value: {}, timestamp: {}", value, timestamp - 100);
            }),
        });
        vec.push(Sender {
            timestamp: 0,
            executor: Box::new(move || {
                log::info!("current value: {}, timestamp: {}", value, timestamp - 10);
            }),
        });
        thread::sleep(time::Duration::from_secs(2));
        vec
    }

    fn finished(&self) -> bool {
        self.finished
    }
}

pub fn create_replayer(config: &Value) -> Box<dyn Replayer> {
    let replayer_type = config["type"].as_str().unwrap();
    match replayer_type {
        "mock" => Box::new(MockReplayer {
            value: 0,
            finished: false,
        }),
        _ => panic!("Unknown replayer type"),
    }
}
