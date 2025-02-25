// use std::collections::VecDeque;

use crate::Process;

pub fn srt(mut procs: Vec<Process>) -> Vec<Process> {
    let mut current_time: u32 = 0;
    let n = procs.len();
    let mut completed = 0;

    while completed < n {
        // Find the process with the smallest remaining time that's already arrived.
        let mut idx = None;
        let mut min_remaining = u32::MAX;
        for (i, p) in procs.iter().enumerate() {
            if p.arrival_time <= current_time && p.remaining_time > 0 {
                if p.remaining_time < min_remaining {
                    min_remaining = p.remaining_time;
                    idx = Some(i);
                }
            }
        }

        if let Some(i) = idx {
            // Record first execution time.
            if procs[i].start_time.is_none() {
                procs[i].start_time = Some(current_time);
                procs[i].response_time = Some(current_time - procs[i].arrival_time);
            }
            // Execute for one time unit.
            procs[i].remaining_time -= 1;
            current_time += 1;
            if procs[i].remaining_time == 0 {
                procs[i].completion_time = Some(current_time);
                completed += 1;
            }
        } else {
            // If no process is ready, move forward in time.
            current_time += 1;
        }
    }
    procs
}





pub fn sjf(mut procs: Vec<Process>) -> Vec<Process> {
    let mut current_time = 0;
    let mut completed = 0;
    let n = procs.len();

    // We’ll track which processes are done
    let mut is_done = vec![false; n];

    while completed < n {
        // Pick process with smallest burst_time among those arrived & not done
        let mut idx = None;
        let mut min_bt = u32::MAX;

        for i in 0..n {
            if !is_done[i] && procs[i].arrival_time <= current_time {
                if procs[i].burst_time < min_bt {
                    min_bt = procs[i].burst_time;
                    idx = Some(i);
                }
            }
        }

        if let Some(i) = idx {
            // Schedule this process
            let start = current_time;
            if procs[i].start_time.is_none() {
                procs[i].start_time = Some(start);
                procs[i].response_time = Some(start - procs[i].arrival_time);
            }
            current_time += procs[i].burst_time;
            procs[i].completion_time = Some(current_time);
            is_done[i] = true;
            completed += 1;
        } else {
            // No process arrived yet, jump in time
            current_time += 1;
        }
    }
    procs
}

/// Earliest Deadline First (non-preemptive, using 'priority_or_deadline' as the deadline)
pub fn edf(mut procs: Vec<Process>) -> Vec<Process> {
    let mut current_time = 0;
    let mut completed = 0;
    let n = procs.len();
    let mut is_done = vec![false; n];

    while completed < n {
        let mut idx = None;
        let mut min_deadline = u32::MAX;

        for i in 0..n {
            if !is_done[i] && procs[i].arrival_time <= current_time {
                // pick process with earliest deadline
                if procs[i].priority_or_deadline.unwrap() < min_deadline {
                    min_deadline = procs[i].priority_or_deadline.unwrap();
                    idx = Some(i);
                }
            }
        }

        if let Some(i) = idx {
            let start = current_time;
            if procs[i].start_time.is_none() {
                procs[i].start_time = Some(start);
                procs[i].response_time = Some(start - procs[i].arrival_time);
            }
            current_time += procs[i].burst_time;
            procs[i].completion_time = Some(current_time);
            is_done[i] = true;
            completed += 1;
        } else {
            current_time += 1;
        }
    }
    procs
}

