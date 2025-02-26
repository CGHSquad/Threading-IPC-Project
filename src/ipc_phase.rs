use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::process::{Command, Stdio};
use os_pipe::pipe;
use libc;

fn ipc_phase() {
    // Create a pipe using os_pipe
    let (mut read_pipe, mut write_pipe) = pipe().expect("Failed to create pipe");

    unsafe {
        match libc::fork() {
            -1 => {
                eprintln!("Fork failed!");
            }
            0 => {
                // Child process: Close the write end and read from the pipe
                drop(write_pipe);

                let mut buffer = String::new();
                read_pipe.read_to_string(&mut buffer).expect("Failed to read from pipe");

                println!("Child received: {}", buffer);
                std::process::exit(0); // Exit successfully
            }
            _ => {
                // Parent process: Close the read end and write to the pipe
                drop(read_pipe);

                let message = "Hello from parent!";
                write_pipe.write_all(message.as_bytes()).expect("Failed to write to pipe");
                println!("Parent sent: {}", message);
            }
        }
    }
}

fn main() {
    ipc_phase();
}

pub(crate) fn run() {
    println!("Starting IPC Phase ...");
    ipc_phase(); // Call the actual logic
    println!("Phase IPC Completed.");
}