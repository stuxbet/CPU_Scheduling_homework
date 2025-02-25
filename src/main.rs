pub mod types;
pub mod algo;
use algo::*;
use types::*;

fn main() {

    let processes = vec![
        Process::new(1, 0, 6, Some(1)),
        Process::new(2, 2, 4, Some(3)),
        Process::new(3, 4, 8, Some(2)),
        Process::new(4, 6, 2, Some(1)),
        Process::new(5, 8, 4, Some(3)),
    ];

    let res = sjf(processes.clone());
    print_results("SJF", &res);

    let res = srt(processes.clone());
    print_results("SRT", &res);

    let res = edf(processes.clone());
    print_results("EDF", &res);


}




pub fn print_results(alg_name: &str,procs:&[Process]) {

    let mut new_proc = procs.to_vec();

    for p in new_proc.iter_mut() {
        let ct = p.completion_time.unwrap();
        let at = p.arrival_time;
        let bt = p.burst_time;

        p.turnaround_time = Some(ct - at);
        p.waiting_time = Some(p.turnaround_time.unwrap() - bt);
        // If response_time wasn't set (shouldn't happen if scheduling is correct), default to waiting_time
        p.response_time.get_or_insert(p.waiting_time.unwrap());
        
    }

    let n = procs.len() as f64;
    let mut total_wait = 0;
    let mut total_tat = 0;
    let mut total_resp = 0;
    let mut finish_time = 0;



    println!("{} Results", alg_name);
    println!("_________________________________________________________");
    println!("ID | Start | Completion | Waiting | Turnaround | Response");

    for p in new_proc {

        let w = p.waiting_time.unwrap_or_else(|| { println!("no waiting time for {}", p.id); 0 });
        let t = p.turnaround_time.unwrap_or_else(|| { println!("no turn around time for {}", p.id); 0 });
        let r = p.response_time.unwrap_or_else(|| { println!("no response time for {}", p.id); 0 });
        let s = p.start_time.unwrap_or_else(|| { println!("no start time for {}", p.id); 0 });
        let c = p.completion_time.unwrap_or_else(|| { println!("no completion time for {}", p.id); 0 });
        total_wait += w;
        total_tat += t;
        total_resp += r;
        if c > finish_time {
            finish_time = c;
        }
        println!(
            "p{} | {:5} | {:10} | {:7} | {:10} | {:8}",
            p.id, s, c, w, t, r
        );

    }
    println!(
        "Avg Waiting: {:.2}, Avg Turnaround: {:.2}, Avg Response: {:.2}",
        total_wait as f64 / n,
        total_tat as f64 / n,
        total_resp as f64 / n
    );


    let cpu_utilization = 100.0 * (finish_time as f64 / finish_time as f64);
    println!("CPU Utilization: {:.2}%", cpu_utilization);

    let throughput = n / finish_time as f64;
    println!("Throughput: {:.2} processes/unit time\n", throughput);
}