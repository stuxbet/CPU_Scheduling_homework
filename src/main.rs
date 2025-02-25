pub mod types;
pub mod algo;
use algo::*;
use types::*;

fn main() {

    let processes = vec![
        Process::new(1, 0, 6, 1),
        Process::new(2, 2, 4, 3),
        Process::new(3, 4, 8, 2),
        Process::new(4, 6, 2, 1),
        Process::new(5, 8, 4, 3),
    ];

    // FCFS
    let fcfs_res = fcfs(processes.clone());
    // print_results("FCFS", &fcfs_res);



    println!("Hello, world!");
}
