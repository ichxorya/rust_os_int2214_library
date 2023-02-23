pub mod preemptive;
pub mod nonpreemptive;

/// `Process` struct.
pub struct Process {
    pub pid: u32,
    pub arrival_time: f32,
    pub burst_time: f32,

    pub remaining_time: f32,
    pub waiting_time: f32,
    pub turn_around_time: f32,
    pub priority: u32,

    pub section_finish_time: f32,

    pub start_time: f32,
    pub finish_time: f32,
}

/// Implement `Clone` trait for `Process` struct.
impl Clone for Process {
    fn clone(&self) -> Process {
        Process {
            pid: self.pid,
            arrival_time: self.arrival_time,
            burst_time: self.burst_time,
            remaining_time: self.remaining_time,
            waiting_time: self.waiting_time,
            turn_around_time: self.turn_around_time,
            priority: self.priority,
            section_finish_time: self.section_finish_time,
            start_time: self.start_time,
            finish_time: self.finish_time,
        }
    }
}

/// Implement `PartialEq` trait for `Process` struct.
impl PartialEq for Process {
    fn eq(&self, other: &Process) -> bool {
        self.arrival_time == other.arrival_time
    }
}

/// Implement `Eq` trait for `Process` struct.
impl Eq for Process {}

/// Implement `PartialOrd` trait for `Process` struct.
impl PartialOrd for Process {
    fn partial_cmp(&self, other: &Process) -> Option<std::cmp::Ordering> {
        self.arrival_time.partial_cmp(&other.arrival_time)
    }
}

/// Implement `Hash` trait for `Process` struct.
impl std::hash::Hash for Process {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pid.hash(state);
    }
}

impl Process {
    /// Constructor for `Process` struct.
    fn new(pid: u32, arrival_time: f32, burst_time: f32) -> Process {
        non_negative_check(arrival_time, burst_time);
        Process {
            pid,
            arrival_time,
            burst_time,
            remaining_time: burst_time,

            waiting_time: 0.0,
            turn_around_time: 0.0,
            priority: 0,
            section_finish_time: 0.0,
            start_time: 0.0,
            finish_time: 0.0,
        }
    }

    /// Constructor for `Process` struct with priority.
    fn new_with_priority(pid: u32, arrival_time: f32, burst_time: f32, priority: u32) -> Process {
        non_negative_check(arrival_time, burst_time);
        Process {
            pid,
            arrival_time,
            burst_time,
            remaining_time: burst_time,

            waiting_time: 0.0,
            turn_around_time: 0.0,
            priority,
            section_finish_time: 0.0,
            start_time: 0.0,
            finish_time: 0.0,
        }
    }
}

fn non_negative_check(arrival_time: f32, burst_time: f32) {
    if (arrival_time < 0.0) || (burst_time < 0.0) {
        panic!("Process parameters must be a non negative real number.");
    }
}

/// `Event` struct.
pub struct Event {
    pub name: String,
    pub start_time: u32,
    pub finish_time: u32,
}