pub mod types;
pub mod algo;
use algo::*;
use types::*;

fn main() {

    let processes = vec![
        Process::new(1, 0, 6, None),
        Process::new(2, 2, 4, None),
        Process::new(3, 4, 8, None),
        Process::new(4, 6, 2, None),
        Process::new(5, 8, 4, None),
    ];

    let fcfs_res = fcfs(processes.clone());
    print_results("FCFS", &fcfs_res);



}
