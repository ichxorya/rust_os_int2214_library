use priority_queue::DoublePriorityQueue;

use super::Process;

pub struct NonpreemptiveScheduler {
    pub processes: Vec<Process>,
    pub finished_processes: Vec<Process>,
}

// Common methods
impl NonpreemptiveScheduler {
    pub fn new(processes: Vec<Process>) -> NonpreemptiveScheduler {
        NonpreemptiveScheduler { processes, finished_processes: Vec::new() }
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
        process.section_finish_time = finish_time;
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
impl NonpreemptiveScheduler {
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
impl NonpreemptiveScheduler {
    // First Come First Serve (FCFS).
    pub fn fcfs(&mut self) {
        // Sort the processes by their arrival time.
        self.sort_by_arrival_time();

        // Calculate the waiting time and turn around time for each process.
        let mut current_time: f64 = self.processes[0].arrival_time;
        (0..self.processes.len()).for_each(|i: usize| {
            // Get the process.
            let mut process: Process = self.processes[i].clone();

            // Calculate the start time, finish time, waiting time and turn around time. 
            let time_tuple: (f64, f64, f64, f64) = self.calculate_time(current_time, &process);

            // Update the current time.
            current_time = time_tuple.1;

            // Update the process to the list.
            self.update_process(&mut process, 
                time_tuple.0, time_tuple.1, 
                time_tuple.2, time_tuple.3);

            // Push the process to the finished processes list.
            self.finished_processes.push(process);
        });

        // Print the result.
        self.print();
    }

    // Shortest Job First (SJF).
    // Ref: https://github.com/KaoSon2004/OS/blob/main/SJF.cs
    pub fn sjf(&mut self) {
        // Sort the processes by their arrival time.
        self.sort_by_arrival_time();

        // A queue to store the processes that have arrived.
        let mut queue: DoublePriorityQueue<Process, u32> = DoublePriorityQueue::new();

        // Start the loop (while the queue or the processes list is not empty).
        let mut current_time: f64 = self.processes[0].arrival_time;
        while !queue.is_empty() || !self.processes.is_empty() {
            // While the processes list is not empty and the first process in the list has not arrived yet.
            while !self.processes.is_empty() && self.processes[0].arrival_time <= current_time {
                // Pop the first process from the processes list.
                let process: Process = self.processes.remove(0);
                let burst_time: f64 = process.burst_time;

                // Add it to the queue.
                queue.push(process, burst_time as u32);
            }

            // If the queue is not empty.
            if !queue.is_empty() {
                // Pop the process with the shortest burst time.
                let mut process: Process = queue.pop_min().unwrap().0;

                // Calculate the start time, finish time, waiting time and turn around time.
                let time_tuple: (f64, f64, f64, f64) = self.calculate_time(current_time, &process);

                // Update the selected process.
                self.update_process(&mut process,
                    time_tuple.0, time_tuple.1, 
                    time_tuple.2, time_tuple.3);

                // Update the current time.
                current_time = time_tuple.1;

                // Add the process to the finished processes list.
                self.finished_processes.push(process); 
            }
            // If the queue is empty: Update the current time.
            else {
                current_time += 0.01;
            }
        }

        // Print the result.
        self.print();
    }

    // Priority Scheduling (PS).
    // Basically SJF but Smallest-Priority-First.
    pub fn ps(&mut self) {
        // Sort the processes by their arrival time.
        self.sort_by_arrival_time();

        // A queue to store the processes that have arrived.
        let mut queue: DoublePriorityQueue<Process, u32> = DoublePriorityQueue::new();

        // Start the loop (while the queue or the processes list is not empty).
        let mut current_time: f64 = self.processes[0].arrival_time;
        while !queue.is_empty() || !self.processes.is_empty() {
            // While the processes list is not empty and the first process in the list has not arrived yet.
            while !self.processes.is_empty() && self.processes[0].arrival_time <= current_time {
                // Pop the first process from the processes list.
                let process: Process = self.processes.remove(0);
                let priority: u32 = process.priority;

                // Add it to the queue.
                queue.push(process, priority as u32);
            }

            // If the queue is not empty.
            if !queue.is_empty() {
                // Pop the process with the shortest burst time.
                let mut process: Process = queue.pop_min().unwrap().0;

                // Calculate the start time, finish time, waiting time and turn around time.
                let time_tuple: (f64, f64, f64, f64) = self.calculate_time(current_time, &process);

                // Update the selected process.
                self.update_process(&mut process,
                    time_tuple.0, time_tuple.1, 
                    time_tuple.2, time_tuple.3);

                // Update the current time.
                current_time = time_tuple.1;

                // Add the process to the finished processes list.
                self.finished_processes.push(process); 
            }
            // If the queue is empty: Update the current time.
            else {
                current_time += 0.01;
            }
        }

        // Print the result.
        self.print();
    }
}

// Test the SJF, cfgtest
#[test]
fn test() {
    let mut scheduler: NonpreemptiveScheduler = NonpreemptiveScheduler::new(
        vec![
            Process::new(1, 0.0, 3.0),
            Process::new(2, 1.0, 0.01),
            Process::new(3, 1.0, 0.01),
            Process::new(4, 1.0, 0.01),
            // Process::new(3, 1.0, 1.211),
        ]
    );


    scheduler.sjf();

}