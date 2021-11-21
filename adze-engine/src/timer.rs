use std::time::Instant;

pub struct Timer<'a> {
    clock: Instant,
    start: u128,
    name: &'a str
}

impl <'a> Timer <'a> {
    pub fn new(name: &'a str) -> Self {
        let clock = Instant::now();
        Timer {
            start: clock.elapsed().as_micros(),
            clock,
            name
        }
    }
}

impl <'a> Drop for Timer <'a> {
    fn drop(&mut self) {
        let end = self.clock.elapsed().as_micros();
        let duration= (end - self.start) as f32 * 0.001;
        println!("{} {}", self.name, duration)
    }
}