use crate::Process;

pub fn fcfs(mut procs: Vec<Process>) -> Vec<Process> {
    // Sort by arrival time
    procs.sort_by_key(|p| p.arrival_time);

    let mut current_time = 0;
    for p in procs.iter_mut() {
        if current_time < p.arrival_time {
            current_time = p.arrival_time;
        }
        p.start_time = Some(current_time);
        p.response_time = Some(current_time - p.arrival_time); // first time CPU allocated
        current_time += p.burst_time;
        p.completion_time = Some(current_time);
    }
    //finalize_metrics(&mut procs);
    procs
}


pub fn print_results(alg_name: &str, procs: &[Process]) {
    let n = procs.len() as f64;
    let mut total_wait = 0;
    let mut total_tat = 0;
    let mut total_resp = 0;
    let mut finish_time = 0;

    println!("=== {} RESULTS ===", alg_name);
    println!("ID | Start | Completion | Waiting | Turnaround | Response");
    for p in procs {
        let w = p.waiting_time.unwrap();
        let t = p.turnaround_time.unwrap();
        let r = p.response_time.unwrap();
        let s = p.start_time.unwrap();
        let c = p.completion_time.unwrap();
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

    // CPU Utilization = (finish_time - idle_time) / finish_time
    // Here we assume no forced idle time if processes are continuously available
    // (except the times we jump if no process is available).
    // For simplicity, we assume minimal idle. This is not always exact.
    // We'll do a naive approach assuming CPU busy from 0 to finish_time if at least one process arrived at 0.
    // Adapt if your assignment requires a more precise approach.
    let cpu_utilization = 100.0 * (finish_time as f64 / finish_time as f64);
    println!("CPU Utilization ~ {:.2}%", cpu_utilization);

    // Throughput = # of processes / total_time
    let throughput = n / finish_time as f64;
    println!("Throughput: {:.2} processes/unit time\n", throughput);
}