pub struct Step {
    start_time: u64,
    cur_time: u64,
    cur_step: u64,
    step_width: u64,
}

impl Step {
    pub fn new(start_time: u64, step_width: u64) -> Self {
        Step {
            start_time,
            cur_time: start_time,
            cur_step: 0,
            step_width,
        }
    }
    pub fn next(&mut self) {
        self.cur_step += 1;
        self.cur_time = self.start_time + self.cur_step * self.step_width;
    }

    pub fn reset(&mut self, start_time: u64, step_width: u64) {
        self.start_time = start_time;
        self.cur_time = start_time;
        self.cur_step = 0;
        self.step_width = step_width;
    }

    pub const fn cur_time(&self) -> u64 {
        self.cur_time
    }

    pub const fn cur_step(&self) -> u64 {
        self.cur_step
    }
}
