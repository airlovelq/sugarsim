use log;

pub struct Sender {
    pub timestamp: u64,
    pub executor: Box<dyn FnMut()>,
}

pub trait SenderController {
    fn exec(&mut self, senders: &mut Vec<Sender>);
}

struct SimpleSenderController {}

impl SenderController for SimpleSenderController {
    fn exec(&mut self, senders: &mut Vec<Sender>) {
        senders.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        for sender in senders.iter_mut() {
            log::info!("step one sender, timestamp: {}", sender.timestamp);
            (sender.executor)();
        }
    }
}

pub fn create_sender_controller(sender_type: &str) -> Box<dyn SenderController> {
    match sender_type {
        "simple" => Box::new(SimpleSenderController {}),
        _ => panic!("Unknown replayer type"),
    }
}
