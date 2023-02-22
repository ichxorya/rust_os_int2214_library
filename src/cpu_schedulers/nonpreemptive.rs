use super::Process;

pub struct NonpreemptiveScheduler {
    pub processes: Vec<Process>,
    pub total_waiting_time: u32,
    pub total_turn_around_time: u32,
    pub average_waiting_time: f32,
    pub average_turn_around_time: f32,
}

// Common methods
impl NonpreemptiveScheduler {
    // Constructor
    fn new(process: Vec<Process>) -> NonpreemptiveScheduler {
        // Panic if the process vector is empty
        if process.is_empty() {
            panic!("The process vector is empty!");
        }

        NonpreemptiveScheduler {
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

    fn calculate_waiting_time(&mut self) {
        let mut waiting_time = 0;
        for i in 0..self.processes.len() {
            self.processes[i].waiting_time = waiting_time;
            waiting_time += self.processes[i].burst_time;
        }
    }

    fn calculate_turn_around_time(&mut self) {
        for i in 0..self.processes.len() {
            self.processes[i].turn_around_time =
                self.processes[i].waiting_time + self.processes[i].burst_time;
        }
    }

    fn calculate_finish_time(&mut self) {
        let mut finish_time = 0;
        for i in 0..self.processes.len() {
            finish_time += self.processes[i].burst_time;
            self.processes[i].finish_time = finish_time;
        }
    }

    fn calculate_total_waiting_time(&mut self) {
        let mut total_waiting_time = 0;
        for i in 0..self.processes.len() {
            total_waiting_time += self.processes[i].waiting_time;
        }
        self.total_waiting_time = total_waiting_time;
    }

    fn calculate_total_turn_around_time(&mut self) {
        let mut total_turn_around_time = 0;
        for i in 0..self.processes.len() {
            total_turn_around_time += self.processes[i].turn_around_time;
        }
        self.total_turn_around_time = total_turn_around_time;
    }

    fn calculate_average_waiting_time(&mut self) {
        let average_waiting_time = self.total_waiting_time as f32 / self.processes.len() as f32;
        self.average_waiting_time = average_waiting_time;
    }

    fn calculate_average_turn_around_time(&mut self) {
        let average_turn_around_time =
            self.total_turn_around_time as f32 / self.processes.len() as f32;
        self.average_turn_around_time = average_turn_around_time;
    }
}

// Visualization
impl NonpreemptiveScheduler {
    pub fn print(&self) {
        println!("Name\tArrival Time\tBurst Time\tPriority\tWaiting Time\tTurn Around Time\tFinish Time");
        for i in 0..self.processes.len() {
            println!(
                "{}\t{}\t\t{}\t\t{}\t\t{}\t\t{}\t\t\t{}",
                self.processes[i].name,
                self.processes[i].arrival_time,
                self.processes[i].burst_time,
                self.processes[i].priority,
                self.processes[i].waiting_time,
                self.processes[i].turn_around_time,
                self.processes[i].finish_time
            );
        }
        println!(
            "Average\t\t\t\t\t\t\t*{}\t\t*{}", 
            self.average_waiting_time, 
            self.average_turn_around_time
        );

        self.gantt_chart();
    }

    // Idea from: https://github.com/marvinjason/CPUScheduler
    pub fn gantt_chart(&self) {
        let mut gantt_chart = "\n\nGantt Chart:\n".to_string();
        let mut time = 0;
        let number_of_processes = self.processes.len();
        if number_of_processes == 1 {
            gantt_chart.push_str(&format!("{}\n", self.processes[0].waiting_time));
            gantt_chart.push_str(&format!("|    {}\n", self.processes[0].name));
            gantt_chart.push_str(&format!("{}\n", self.processes[0].finish_time));
        } else {
            gantt_chart.push_str(&format!("{}\n", self.processes[0].waiting_time));
            gantt_chart.push_str(&format!("|    {}\n", self.processes[0].name));

            for i in 1..self.processes.len() {
                time += self.processes[i - 1].burst_time;
                gantt_chart.push_str(&format!("{}\n", time));
                gantt_chart.push_str(&format!("|    {}\n", self.processes[i].name));
            }
            gantt_chart.push_str(&format!("{}\n", self.processes[number_of_processes - 1].finish_time));
        }
        println!("{}", gantt_chart);
    }
}

// Algorithms
impl NonpreemptiveScheduler {
    // First Come First Serve (FCFS).
    pub fn fcfs(&mut self) {
        self.sort_by_arrival_time();
        self.calculate_waiting_time();
        self.calculate_turn_around_time();
        self.calculate_total_waiting_time();
        self.calculate_total_turn_around_time();
        self.calculate_average_waiting_time();
        self.calculate_average_turn_around_time();
        self.calculate_finish_time();
        self.print();
    }

    // Priority Scheduling (lowest number has highest priority).
    pub fn priority(&mut self) {
        self.sort_by_priority();
        self.fcfs();
    }

    // Reverse Priority Scheduling (highest number has highest priority).
    pub fn reverse_priority(&mut self) {
        self.sort_by_priority_reverse();
        self.fcfs();
    }

    // Shortest Job First (SJF).
    pub fn shortest_job_first(&mut self) {
        self.sort_by_burst_time();
        self.fcfs();
    }
}

/**
 * Test the algorithms using test cfg
 */
#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_fcfs() {
        let mut scheduler = NonpreemptiveScheduler::new(
            vec!(
                Process::new("P1", 0, 24, 3),
                Process::new("P2", 0, 3, 2),
                Process::new("P3", 0, 24, 0),
            )
        );
            
        scheduler.priority();
    }
}