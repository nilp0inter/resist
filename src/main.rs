use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;
use std::io::{self, BufRead, Write};
use std::os::unix::io::{FromRawFd, RawFd};
use nix::fcntl::{fcntl, FcntlArg};

fn is_fd_valid(fd: RawFd) -> bool {
    fcntl(fd, FcntlArg::F_GETFD).is_ok()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut probabilities = Vec::new();
    let mut outputs = Vec::new();

    // Skip the first argument (program name)
    for (index, arg) in args.iter().enumerate().skip(1) {
        let conductance: f64 = arg.parse().expect("Invalid conductance value");
        probabilities.push(conductance);

        // File descriptors start from 1 (stdout) and 2 (stderr)
        let fd: RawFd = 1 + index as RawFd - 1;
        if is_fd_valid(fd) {
            let file = unsafe { std::fs::File::from_raw_fd(fd) };
            outputs.push(file);
        } else {
            eprintln!("Warning: File descriptor {} is not valid and will be skipped.", fd);
            probabilities[index - 1] = 0.0; // Set probability to 0 for invalid FDs
        }
    }

    let dist = if probabilities.iter().any(|&p| p > 0.0) {
        Some(WeightedIndex::new(&probabilities).expect("Invalid probabilities"))
    } else {
        None
    };

    let mut rng = thread_rng();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        if let Some(ref dist) = dist {
            let chosen_output = dist.sample(&mut rng);
            writeln!(outputs[chosen_output], "{}", line)?;
        }
    }

    Ok(())
}
