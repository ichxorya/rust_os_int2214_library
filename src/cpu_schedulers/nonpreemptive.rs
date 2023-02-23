use priority_queue::PriorityQueue;

use super::Process;

pub struct NonpreemptiveScheduler {
    pub processes: Vec<Process>,
}

// Common methods
impl NonpreemptiveScheduler {
    pub fn new(processes: Vec<Process>) -> NonpreemptiveScheduler {
        NonpreemptiveScheduler { processes }
    }

    pub fn sort_by_arrival_time(&mut self) {
        self.processes.sort_by(|a, b| a.arrival_time.partial_cmp(&b.arrival_time).unwrap());
    }

    pub fn update_process(&mut self, index: usize, start_time: f32, finish_time: f32, waiting_time: f32, turn_around_time: f32) {
        self.processes[index].start_time = start_time;
        self.processes[index].finish_time = finish_time;
        self.processes[index].waiting_time = waiting_time;
        self.processes[index].turn_around_time = turn_around_time;
    }
}

// Visualization
impl NonpreemptiveScheduler {
    pub fn print(&mut self) {
        println!("Name\t\tArrival Time\tBurst Time\tWaiting Time\tTurn Around Time\tFinish Time");
        for i in 0..self.processes.len() {
            println!(
                "P{}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t{:.2}\t\t\t{:.2}",
                self.processes[i].pid,
                self.processes[i].arrival_time,
                self.processes[i].burst_time,
                self.processes[i].waiting_time,
                self.processes[i].turn_around_time,
                self.processes[i].finish_time
            );
        }
        // Calculate average waiting time and average turn around time using list comprehension.
        let average_waiting_time: f32 = self
            .processes
            .iter()
            .map(|process| process.waiting_time)
            .sum::<f32>()
            / self.processes.len() as f32;
        
        let average_turn_around_time: f32 = self
            .processes
            .iter()
            .map(|process| process.turn_around_time)
            .sum::<f32>()
            / self.processes.len() as f32;

        println!(
            "Average:\t\t\t\t\t*{:.2}\t\t*{:.2}", 
            average_waiting_time,
            average_turn_around_time
        );

        self.gantt_chart();
    }

    // Idea from: https://github.com/marvinjason/CPUScheduler
    pub fn gantt_chart(&self) {
        // let mut gantt_chart = "\n\nGantt Chart:\n".to_string();
        // let mut time = 0;
        // let number_of_processes = self.processes.len();
        // if number_of_processes == 1 {
        //     gantt_chart.push_str(&format!("{}\n", self.processes_waiting_time[0]));
        //     gantt_chart.push_str(&format!("|    {}\n", self.processes[0].pid));
        //     gantt_chart.push_str(&format!("{}\n", self.processes_finish_time[0]));
        // } else {
        //     gantt_chart.push_str(&format!("{}\n", self.processes_waiting_time[0]));
        //     gantt_chart.push_str(&format!("|    {}\n", self.processes[0].pid));

        //     for i in 1..self.processes.len() {
        //         time += self.processes[i - 1].burst_time;
        //         gantt_chart.push_str(&format!("{}\n", time));
        //         gantt_chart.push_str(&format!("|    {}\n", self.processes[i].pid));
        //     }
        //     gantt_chart.push_str(&format!("{}\n", self.processes_finish_time[number_of_processes - 1]));
        // }
        // println!("{}", gantt_chart);
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
        for i in 0..self.processes.len() {
            // Start time = current time.
            let start_time: f32 = current_time;

            // Finish time = start time + burst time.
            let finish_time: f32 = start_time + self.processes[i].burst_time;

            // Turn around time = finish time - arrival time.
            let turn_around_time: f32 = finish_time - self.processes[i].arrival_time;
            
            // Waiting time = turn around time - burst time.
            let waiting_time: f32 = turn_around_time - self.processes[i].burst_time;

            // Update the current time.
            current_time = finish_time;

            // Update the process.
            self.update_process(i, start_time, finish_time, waiting_time, turn_around_time);
        }

        // Print the result.
        self.print();
    }

    // Shortest Job First (SJF).
    // Ref: https://github.com/KaoSon2004/OS/blob/main/SJF.cs
    pub fn sjf(&mut self) {
        // self.sort_by_arrival_time();
        
        // // Priority queue for shortest job first.
        // let mut queue: PriorityQueue<Process, u32> = PriorityQueue::new();
        // let mut ans: Vec<Process> = Vec::new();

        // let mut current_time = self.processes[0].arrival_time;

        // // While there are processes in the queue, or there are processes in the list.
        // while !queue.is_empty() || !self.processes.is_empty() {
        //     // While there are processes in the list, and they have not arrived yet.
        //     while !self.processes.is_empty() && self.processes[0].arrival_time <= current_time {
        //         // Add the process to the queue.
        //         queue.push(self.processes.remove(0), self.processes[0].burst_time);

        //         // Remove the process from the list.
        //         self.processes.remove(0);
        //     }

        //     // If the queue is not empty, then we can run the process.
        //     if !queue.is_empty() {
        //         // Get the process from the queue.
        //         let mut process = queue.pop().unwrap().0;

        //         // Process "start time" is its "arrival time".
        //         process.arrival_time = current_time;

        //         // Increment the current time.
        //         current_time += process.burst_time;

        //         // Add the process to the answer queue.
        //         ans.push(process);
        //     } else {
        //         // If the queue is empty, then we can just increment the current time.
        //         current_time += 1;
        //     }
        // }

        // // Turn the answer queue into the processes vector.
        // self.processes = ans;
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
            Process::new(4, 0.1, 3.0),
            Process::new(5, 4.0, 4.0),
        ]);
        scheduler.fcfs();
    }
}