use priority_queue::DoublePriorityQueue;

use super::{Process, more_than_two_decimal_places, Event};

pub struct PreemptiveScheduler {
    pub processes: Vec<Process>,
    pub event_list: Vec<Event>,
    pub finished_processes: Vec<Process>,
}

// Common methods
impl PreemptiveScheduler {
    pub fn new(processes: Vec<Process>) -> PreemptiveScheduler {
        PreemptiveScheduler { processes, finished_processes: Vec::new(), event_list: Vec::new() }
    }

    pub fn sort_by_arrival_time(&mut self) {
        self.processes.sort_by(|a, b| a.arrival_time.partial_cmp(&b.arrival_time).unwrap());
    }

    pub fn update_process(&mut self, process: &mut Process, start_time: f64, finish_time: f64, waiting_time: f64, turn_around_time: f64) {
        process.start_time = start_time;
        process.finish_time = finish_time;
        process.waiting_time = waiting_time;
        process.turn_around_time = turn_around_time;
        process.remaining_time = 0.0;
        todo!()
    }

    pub fn calculate_time(&self, current_time: f64, process: &Process) -> (f64, f64, f64, f64) {
        // Start time = current time.
        let start_time: f64 = current_time;

        // Finish time = start time + burst time.
        let finish_time: f64 = start_time + process.burst_time;

        // Turn around time = finish time - arrival time.
        let turn_around_time: f64 = finish_time - process.arrival_time;

        // Waiting time = turn around time - burst time.
        let waiting_time: f64 = finish_time - process.arrival_time - process.burst_time;

        // Return the calculated times.
        (start_time, finish_time, waiting_time, turn_around_time)
    }
}

// Visualization
impl PreemptiveScheduler {
    pub fn print(&mut self) {
        // Define a processes variable to store the finished processes.
        let processes: Vec<Process> = self.finished_processes.clone();

        println!("Name\t\tArrival Time\tBurst Time\tTurn Around Time\tWaiting Time\tFinish Time");
        for i in 0..processes.len() {
            println!(
                "P{}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{:.2}\t\t\t{:.2}",
                processes[i].pid,
                processes[i].arrival_time,
                processes[i].burst_time,
                processes[i].turn_around_time,
                processes[i].waiting_time,
                processes[i].finish_time
            );
        }
        // Calculate average waiting time and average turn around time using list comprehension.
        let average_waiting_time: f64 = self
            .processes
            .iter()
            .map(|process| process.waiting_time)
            .sum::<f64>()
            / processes.len() as f64;
        
        let average_turn_around_time: f64 = self
            .processes
            .iter()
            .map(|process| process.turn_around_time)
            .sum::<f64>()
            / processes.len() as f64;

        println!(
            "Average:\t\t\t\t\t*{:.2}\t\t\t*{:.2}", 
            average_turn_around_time,
            average_waiting_time,
        );

        self.gantt_chart(&processes);
    }

    // Idea from: https://github.com/marvinjason/CPUScheduler
    pub fn gantt_chart(&self, processes: &Vec<Process>) {
        let mut gantt_chart: String = "\n\nGantt Chart:\n".to_string();
        let mut time: f64 = 0.0;
        let number_of_processes = processes.len();
        if number_of_processes == 1 {
            gantt_chart.push_str(&format!("{}\n", processes[0].waiting_time));
            gantt_chart.push_str(&format!("|    P{}\n", processes[0].pid));
            gantt_chart.push_str(&format!("{}\n", processes[0].finish_time));
        } else {
            gantt_chart.push_str(&format!("{}\n", processes[0].waiting_time));
            gantt_chart.push_str(&format!("|    P{}\n", processes[0].pid));

            for i in 1..processes.len() {
                time += processes[i - 1].burst_time;
                gantt_chart.push_str(&format!("{}\n", time));
                gantt_chart.push_str(&format!("|    P{}\n", processes[i].pid));
            }
            gantt_chart.push_str(&format!("{}\n", processes[number_of_processes - 1].finish_time));
        }
        println!("{}", gantt_chart);
    }
}

// Algorithms
impl PreemptiveScheduler {
    // Round Robin (RR).
    pub fn rr(&mut self, time_quantum: f64) {
        // Check if time quantum is valid.
        if time_quantum <= 0.0 || more_than_two_decimal_places(time_quantum) {
            panic!("Invalid time quantum!");
        }

        // Count the number of processes.
        let number_of_processes: usize = self.processes.len();

        // Sort the processes by arrival time.
        self.sort_by_arrival_time();

        // The current time.
        let mut current_time: f64 = 0.0;

        // While there are still processes to be executed.
        while !self.processes.is_empty() {
            // Get the first process.
            let mut current_process: Process = self.processes.remove(0);

            // Alter the current process' remaining time.
            let time_run: f64 = if current_process.remaining_time > time_quantum {
                time_quantum
            } else {
                current_process.remaining_time
            };

            current_process.remaining_time -= time_run;

            // Calculate the times.
            current_time += time_run;
            current_process.waiting_time += current_time - time_run - current_process.section_finish_time;
            current_process.section_finish_time = current_time;
        }
    }

    // Shortest Remaining Time First (SRTF).
    // Basically SJF but with preemption.
    pub fn srtf(&mut self) {
        
    }

    // Priority Scheduling (PS).
    // Basically SJF but Smallest-Priority-First.
    pub fn ps(&mut self) {
        
    }
}