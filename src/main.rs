//use std::env;
//use std::fs::File;
//#[warn(unused_imports)]
//fn main() {
//    // --snip--
//    //println!("In file {file_path}");
//
//    match File::open(String::from("/proc")) {
//    Ok(mut file) => {
//        let mut contents = String::new();
//        file.read_to_string(&mut contents)?;
//        println!("File contents: {}", contents);
//    }
//    Err(e) => {
//        eprintln!("Failed to open the file: {}", e);
//    }
//}
//
//}
/*
use std::fs::File;
use std::io::{self, Read}; // Import the Read trait

fn main() -> io::Result<()> {
    let path = "/proc/stat"; // Make sure this path points to a file, not a directory
    let mut file = File::open(path)?;
    
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // Now read_to_string is in scope

    println!("File contents: {}", contents);

    Ok(())
}
*/
use std::fs::File;
use std::io::{self, Read, BufRead, BufReader};

fn read_cpu_stats() -> io::Result<Vec<u64>> {
    let file = File::open("/proc/stat")?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("cpu ") {
            let values: Vec<u64> = line
                .split_whitespace()
                .skip(1) // Skip the "cpu" prefix
                .map(|s| s.parse().unwrap())
                .collect();
            return Ok(values);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "CPU stats not found"))
}

fn calculate_cpu_usage(prev: &[u64], curr: &[u64]) -> f64 {
    let prev_idle = prev[3] + prev[4];
    let curr_idle = curr[3] + curr[4];

    let prev_total: u64 = prev.iter().sum();
    let curr_total: u64 = curr.iter().sum();

    let total_diff = curr_total - prev_total;
    let idle_diff = curr_idle - prev_idle;

    let usage = (total_diff - idle_diff) as f64 / total_diff as f64 * 100.0;
    usage
}

fn main() -> io::Result<()> {
   loop {let prev_stats = read_cpu_stats()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    let curr_stats = read_cpu_stats()?;

    let usage = calculate_cpu_usage(&prev_stats, &curr_stats);
    println!("CPU Usage: {:.2}%", usage);
}
    Ok(())
}
