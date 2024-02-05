use std::{fs, time::Instant, process::Command};


// Note this includes the time it takes to compile the code, because each day is run as a separate binary.
fn main() {
    let mut total_time = 0;
    
    let days = fs::read_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bin/")).unwrap()
        .filter_map(|p| p.map(|e| e.path()).ok())
        .filter(|p| {
            p.extension().map(|e| e == "rs").unwrap_or(false)
        })
        .filter_map(|p| p.file_stem().map(|s| s.to_str().unwrap().to_string()))
        .filter(|day| day.chars().all(|c| c.is_ascii_digit()))
        .collect::<Vec<String>>();

    for day in days {
      let time_start = Instant::now();

      let cmd = Command::new("cargo").args(["run", "--bin", &day]).output().ok();

      if let Some(output) = cmd {
        if output.status.success() {
          let run_time = time_start.elapsed().as_micros();
            total_time += run_time;
          println!("Day {} ran in {}ms", day, run_time / 1000);
        }
      }
    }

    println!("\nTotal time: {}ms", total_time / 1000);

    ();
}