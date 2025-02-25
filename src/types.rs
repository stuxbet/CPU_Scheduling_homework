use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct Process {
    pub id: usize,
    pub arrival_time: u32,
    pub burst_time: u32,
    pub remaining_time: u32,
    pub priority_or_deadline: u32, // For EDF
    pub start_time: Option<u32>,
    pub completion_time: Option<u32>,
    pub response_time: Option<u32>,
    pub waiting_time: Option<u32>,
    pub turnaround_time: Option<u32>,
}

impl Process {
    pub fn new(id: usize, arrival_time: u32, burst_time: u32, priority_or_deadline: u32) -> Self {
        Process {
            id,
            arrival_time,
            burst_time,
            remaining_time: burst_time,
            priority_or_deadline,
            start_time: None,
            completion_time: None,
            response_time: None,
            waiting_time: None,
            turnaround_time: None,
        }
    }
}