use super::Process;

pub struct PreemptiveScheduler {
    pub processes: Vec<Process>,
    pub total_waiting_time: u32,
    pub total_turn_around_time: u32,
    pub average_waiting_time: f32,
    pub average_turn_around_time: f32,
}

// Common methods
impl PreemptiveScheduler {
    // Constructor
    fn new(process: Vec<Process>) -> PreemptiveScheduler {
        // Panic if the process vector is empty
        if process.is_empty() {
            panic!("The process vector is empty!");
        }

        PreemptiveScheduler {
            processes: process,
            total_waiting_time: 0,
            total_turn_around_time: 0,
            average_waiting_time: 0.0,
            average_turn_around_time: 0.0,
        }
    }

    // Methods
    fn sort_by_arrival_time(&mut self) {
        self.processes.sort_by(|a, b| a.arrival_time.cmp(&b.arrival_time));
    }

    fn sort_by_burst_time(&mut self) {
        self.processes.sort_by(|a, b| a.burst_time.cmp(&b.burst_time));
    }

    fn sort_by_priority(&mut self) {
        self.processes.sort_by(|a, b| a.priority.cmp(&b.priority));
    }

    fn sort_by_priority_reverse(&mut self) {
        self.processes.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
}

// Visualization

// Algorithms
impl PreemptiveScheduler {
    // Shortest Remaining Time First (SRTF). This is the preemptive version of SJF.
    pub fn srtf(&mut self) {
        
    }
}

/** Event. */
pub struct Event {
    pub name: String,
    pub start_time: u32,
    pub finish_time: u32,
}