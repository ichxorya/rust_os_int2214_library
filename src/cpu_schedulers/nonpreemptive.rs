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

    pub fn update_process(&mut self, process: &mut Process, start_time: f32, finish_time: f32, waiting_time: f32, turn_around_time: f32) {
        process.start_time = start_time;
        process.finish_time = finish_time;
        process.waiting_time = waiting_time;
        process.turn_around_time = turn_around_time;
    }

    pub fn calculate_time(&mut self, current_time: f32, process: &Process) -> (f32, f32, f32, f32) {
        // Start time = current time.
        let start_time: f32 = current_time;

        // Finish time = start time + burst time.
        let finish_time: f32 = start_time + process.burst_time;

        // Turn around time = finish time - arrival time.
        let turn_around_time: f32 = finish_time - process.arrival_time;

        // Waiting time = turn around time - burst time.
        let waiting_time: f32 = finish_time - process.arrival_time - process.burst_time;

        // Return the calculated times.
        (start_time, finish_time, waiting_time, turn_around_time)
    }
}

// Visualization
impl NonpreemptiveScheduler {
    pub fn print(&mut self) {
        println!("Name\t\tArrival Time\tBurst Time\tTurn Around Time\tWaiting Time\tFinish Time");
        for i in 0..self.finished_processes.len() {
            println!(
                "P{}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{:.2}\t\t\t{:.2}",
                self.finished_processes[i].pid,
                self.finished_processes[i].arrival_time,
                self.finished_processes[i].burst_time,
                self.finished_processes[i].turn_around_time,
                self.finished_processes[i].waiting_time,
                self.finished_processes[i].finish_time
            );
        }
        // Calculate average waiting time and average turn around time using list comprehension.
        let average_waiting_time: f32 = self
            .finished_processes
            .iter()
            .map(|process| process.waiting_time)
            .sum::<f32>()
            / self.finished_processes.len() as f32;
        
        let average_turn_around_time: f32 = self
            .finished_processes
            .iter()
            .map(|process| process.turn_around_time)
            .sum::<f32>()
            / self.finished_processes.len() as f32;

        println!(
            "Average:\t\t\t\t\t*{:.2}\t\t\t*{:.2}", 
            average_turn_around_time,
            average_waiting_time,
        );

        self.gantt_chart();
    }

    // Idea from: https://github.com/marvinjason/CPUScheduler
    pub fn gantt_chart(&self) {
        let mut gantt_chart: String = "\n\nGantt Chart:\n".to_string();
        let mut time: f32 = 0.0;
        let number_of_processes = self.finished_processes.len();
        if number_of_processes == 1 {
            gantt_chart.push_str(&format!("{}\n", self.finished_processes[0].waiting_time));
            gantt_chart.push_str(&format!("|    P{}\n", self.finished_processes[0].pid));
            gantt_chart.push_str(&format!("{}\n", self.finished_processes[0].finish_time));
        } else {
            gantt_chart.push_str(&format!("{}\n", self.finished_processes[0].waiting_time));
            gantt_chart.push_str(&format!("|    P{}\n", self.finished_processes[0].pid));

            for i in 1..self.processes.len() {
                time += self.finished_processes[i - 1].burst_time;
                gantt_chart.push_str(&format!("{}\n", time));
                gantt_chart.push_str(&format!("|    P{}\n", self.finished_processes[i].pid));
            }
            gantt_chart.push_str(&format!("{}\n", self.finished_processes[number_of_processes - 1].finish_time));
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
        let mut current_time: f32 = self.processes[0].arrival_time;
        (0..self.processes.len()).for_each(|i: usize| {
            // Get the process.
            let mut process: Process = self.processes[i].clone();

            // Calculate the start time, finish time, waiting time and turn around time. 
            let time_tuple: (f32, f32, f32, f32) = self.calculate_time(current_time, &process);

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
        let mut current_time: f32 = self.processes[0].arrival_time;
        while !queue.is_empty() || !self.processes.is_empty() {
            // While the processes list is not empty and the first process in the list has not arrived yet.
            while !self.processes.is_empty() && self.processes[0].arrival_time <= current_time {
                // Pop the first process from the processes list.
                let process: Process = self.processes.remove(0);
                let burst_time: f32 = process.burst_time;

                // Add it to the queue.
                queue.push(process, burst_time as u32);
            }

            // If the queue is not empty.
            if !queue.is_empty() {
                // Pop the process with the shortest burst time.
                let mut process: Process = queue.pop_min().unwrap().0;

                // Calculate the start time, finish time, waiting time and turn around time.
                let time_tuple: (f32, f32, f32, f32) = self.calculate_time(current_time, &process);

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
        
        // A vector to store the processes that have finished.
        let mut finished_processes: Vec<Process> = Vec::new();

        // Start the loop (while the queue or the processes list is not empty).
        let mut current_time: f32 = self.processes[0].arrival_time;
        while !queue.is_empty() || !self.processes.is_empty() {
            // While the processes list is not empty and the first process in the list has not arrived yet.
            while !self.processes.is_empty() && self.processes[0].arrival_time <= current_time {
                // Pop the first process from the processes list.
                let process: Process = self.processes.remove(0);
                let priority: u32 = process.priority;

                // Add it to the queue.
                queue.push(process, priority);
            }

            // If the queue is not empty.
            if !queue.is_empty() {
                // Pop the process with the smallest priority.
                let mut process: Process = queue.pop_min().unwrap().0;

                // Calculate the start time, finish time, waiting time and turn around time.
                let time_tuple: (f32, f32, f32, f32) = self.calculate_time(current_time, &process);

                // Update the selected process.
                self.update_process(&mut process, 
                    time_tuple.0, time_tuple.1, 
                    time_tuple.2, time_tuple.3);

                // Update the current time.
                current_time = time_tuple.1;

                // Add the process to the finished processes list.
                finished_processes.push(process); 
            }
            // If the queue is empty: Update the current time.
            else {
                current_time += 0.01;
            }
        }

        // Update the processes list.
        self.processes = finished_processes;

        // Print the result.
        self.print();
    }
}

// Tests
#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test() {
        let mut scheduler = NonpreemptiveScheduler::new(vec![
            // 4 1 2 5 3
            Process::new(1, 2.0, 6.0),
            Process::new(2, 5.0, 2.0),
            Process::new(3, 1.0, 8.0),
            Process::new(4, 0.0, 3.0),
            Process::new(5, 4.0, 4.0),
        ]);
        scheduler.sjf();
        
        // scheduler = NonpreemptiveScheduler::new(vec![
        //     // 4 3 1 5 2
        //     Process::new(1, 2.0, 6.0),
        //     Process::new(2, 5.0, 2.0),
        //     Process::new(3, 1.0, 8.0),
        //     Process::new(4, 0.0, 3.0),
        //     Process::new(5, 4.0, 4.0),
        // ]);
        // scheduler.fcfs();
    }

    #[test]
    fn test_ps() {
        let mut scheduler = NonpreemptiveScheduler::new(vec![
            // ???
            Process::new_with_priority(1, 0.0, 6.0, 5),
            Process::new_with_priority(2, 0.0, 2.0, 4),
            Process::new_with_priority(3, 0.0, 8.0, 3),
            Process::new_with_priority(4, 0.0, 3.0, 2),
            Process::new_with_priority(5, 0.0, 4.0, 1),
        ]);

        scheduler.ps();
    }
}