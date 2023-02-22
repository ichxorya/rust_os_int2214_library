pub mod preemptive;
pub mod nonpreemptive;

pub struct Process {
    pub name: String,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub priority: u32,
    pub waiting_time: u32,
    pub turn_around_time: u32,
    pub remaining_time: u32,
    pub finish_time: u32,
}

impl Process {
    // Constructor
    fn new(name: &str, arrival_time: u32, burst_time: u32, priority: u32) -> Process {
        Process {
            name: name.to_string(),
            arrival_time,
            burst_time,
            priority,
            // To be calculated later.
            waiting_time: 0,
            turn_around_time: 0,
            remaining_time: burst_time,
            finish_time: burst_time,
        }
    }
}