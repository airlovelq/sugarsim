use log;
use serde_json::Value;
use std::any::Any;
pub trait CallBacker {
    fn wait(&mut self) -> Box<dyn Any>;
}

struct MockCallBacker {
    value: i32,
}

impl CallBacker for MockCallBacker {
    fn wait(&mut self) -> Box<dyn Any> {
        self.value += 1;
        log::info!("callback received value: {}", self.value);
        return Box::new(self.value);
    }
}

pub fn create_callbacker(config: &Value) -> Box<dyn CallBacker> {
    let callbacker_type = config["type"].as_str().unwrap();
    match callbacker_type {
        "mock" => Box::new(MockCallBacker { value: 15 }),
        _ => panic!("Unknown callbacker type"),
    }
}
