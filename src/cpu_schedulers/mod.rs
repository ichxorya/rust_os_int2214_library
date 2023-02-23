pub mod preemptive;
pub mod nonpreemptive;

use std::fmt::{Debug, Display};

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

/// Implement `Debug` trait for `Process` struct.
impl Debug for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Process")
            .field("pid", &self.pid)
            .field("arrival_time", &self.arrival_time)
            .field("burst_time", &self.burst_time)
            .field("remaining_time", &self.remaining_time)
            .field("waiting_time", &self.waiting_time)
            .field("turn_around_time", &self.turn_around_time)
            .field("priority", &self.priority)
            .field("section_finish_time", &self.section_finish_time)
            .field("start_time", &self.start_time)
            .field("finish_time", &self.finish_time)
            .finish()
    }
}

/// Implement `Display` trait for `Process` struct.
impl Display for Process {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "P{}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{:.2}",
        //     self.pid, self.arrival_time, self.burst_time, self.waiting_time, self.turn_around_time, self.finish_time
        // )
        write!(
            f,
            "P{}", self.pid
        )
    }
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
    /// Compare `Process` structs by priority. If priorities are equal, compare by burst time.
    fn partial_cmp(&self, other: &Process) -> Option<std::cmp::Ordering> {
        if self.priority == other.priority {
            Some(self.burst_time.partial_cmp(&other.burst_time).unwrap())
        } else {
            Some(self.priority.partial_cmp(&other.priority).unwrap())
        }
    }
}

/// Implement `Ord` trait for `Process` struct.
impl Ord for Process {
    /// Compare `Process` structs by priority. If priorities are equal, compare by burst time.
    fn cmp(&self, other: &Process) -> std::cmp::Ordering {
        if self.priority == other.priority {
            self.burst_time.partial_cmp(&other.burst_time).unwrap()
        } else {
            self.priority.partial_cmp(&other.priority).unwrap()
        }
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
        time_check(arrival_time, burst_time);
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
        time_check(arrival_time, burst_time);
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

fn time_check(arrival_time: f32, burst_time: f32) {
    let negative: bool = (arrival_time < 0.0) || (burst_time < 0.0);
    let more_than_two_decimal_places: bool = 
    (arrival_time.fract() * 100.0).round() / 100.0 != arrival_time.fract()
    || (burst_time.fract() * 100.0).round() / 100.0 != burst_time.fract();
    if negative || more_than_two_decimal_places
    {
        panic!("Process parameters must be a non negative real number with less than 3 decimal places.");
    }
}

/// `Event` struct.
pub struct Event {
    pub name: String,
    pub start_time: u32,
    pub finish_time: u32,
}