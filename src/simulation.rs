use crate::callbacker;
use crate::replayer;
use crate::sender;
use crate::step::Step;

use log;
use serde_json::Value;
use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct Simulation {
    start_time: u64,
    end_time: u64,
    step: Step,
    pub aborted: Arc<AtomicBool>,
    replayers: Vec<Box<dyn replayer::Replayer>>,
    callbacker: Option<Box<dyn callbacker::CallBacker>>,
    backdata: Option<Box<dyn Any>>,
    sender_controller: Option<Box<dyn sender::SenderController>>,
}

impl Simulation {
    pub fn new() -> Self {
        Simulation {
            start_time: 0,
            end_time: 0,
            step: Step::new(0, 0),
            aborted: Arc::new(AtomicBool::new(false)),
            replayers: Vec::new(),
            callbacker: None,
            backdata: None,
            sender_controller: None,
        }
    }

    pub fn init(&mut self, config: &Value) -> bool {
        log::info!("config is {}", config.to_string());
        let s_start_time = config["start_time"].as_str().unwrap();
        self.start_time = s_start_time.parse::<u64>().unwrap();
        log::info!("start time is {}", self.start_time);
        if let Some(end_time) = config.get("end_time") {
            self.end_time = end_time.as_u64().unwrap();
            log::info!("end time exists: {}", self.end_time);
        }
        let s_step_width = config["step_width"].as_str().unwrap();
        let step_width = s_step_width.parse::<u64>().unwrap();
        log::info!("step width is {}", step_width);
        self.step.reset(self.start_time, step_width);
        if let Some(replayer_items) = config.get("replayer_list") {
            for replayer_item in replayer_items.as_array().unwrap() {
                let mut replayer: Box<dyn replayer::Replayer> =
                    replayer::create_replayer(&replayer_item);
                let res = replayer.init(config);
                if !res {
                    log::error!(
                        "create replayer failed, config: {}",
                        replayer_item.to_string()
                    );
                    return false;
                }
                log::info!(
                    "create replayer successfully, config: {}",
                    replayer_item.to_string()
                );
                self.replayers.push(replayer);
            }
        } else {
            log::error!("lack of replayer list definition!");
            return false;
        }
        if let Some(callbacker_item) = config.get("callbacker") {
            self.callbacker = Some(callbacker::create_callbacker(&callbacker_item));
            log::info!(
                "create callback successfully, config: {}",
                callbacker_item.to_string()
            );
        }
        if let Some(sender_controller_item) = config.get("sender_controller") {
            let sender_controller_type = sender_controller_item["type"].as_str().unwrap();
            self.sender_controller = Some(sender::create_sender_controller(sender_controller_type));
            log::info!(
                "create sender controller successfully, config: {}",
                sender_controller_item.to_string()
            );
        } else {
            log::error!("sender controller not defined!");
            return false;
        }
        true
    }

    pub fn run(&mut self) {
        loop {
            if self.aborted.load(Ordering::SeqCst) {
                log::info!("receive abort signal, simulator aborted");
                break;
            }
            if self.is_replayers_finished() {
                log::info!("all replayer finished, simulator finished");
                break;
            }
            log::info!(
                "step into index {}, timestamp: {}",
                self.step.cur_step(),
                self.step.cur_time()
            );
            let mut sender_collector: Vec<sender::Sender> = Vec::new();
            for replayer in self.replayers.iter_mut() {
                let senders = replayer.step(&self.step, &self.backdata);
                sender_collector.extend(senders);
            }
            if self.sender_controller.is_some() {
                self.sender_controller
                    .as_mut()
                    .unwrap()
                    .exec(&mut sender_collector);
            }
            if self.callbacker.is_some() {
                self.backdata = Some(self.callbacker.as_mut().unwrap().wait());
            }
            self.step.next();
        }
    }

    fn is_replayers_finished(&self) -> bool {
        let mut finished: bool = true;
        for replayer in self.replayers.iter() {
            finished = finished && replayer.finished();
        }
        finished
    }
}
