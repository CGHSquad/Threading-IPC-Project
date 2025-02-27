use std::io::{Read, Write}; // Import traits for reading and writing
use std::os::unix::io::AsRawFd; // For raw file descriptors
use std::process::{Command, Stdio}; // For process management
use os_pipe::pipe; // Library for pipe creation
use libc; // For using system calls like fork()

fn ipc_phase() {
    // Create a pipe using os_pipe library
    let (mut read_pipe, mut write_pipe) = pipe().expect("Failed to create pipe");

    unsafe {
        match libc::fork() {
            -1 => {
                // Fork failed
                eprintln!("Fork failed!");
            }
            0 => {
                // Child process: Close the write end and read from the pipe
                drop(write_pipe); // Child doesn't need to write

                let mut buffer = String::new();
                read_pipe.read_to_string(&mut buffer).expect("Failed to read from pipe");

                // Print the message received from parent
                println!("Child received: {}", buffer);
                std::process::exit(0); // Exit successfully
            }
            _ => {
                // Parent process: Close the read end and write to the pipe
                drop(read_pipe); // Parent doesn't need to read

                let message = "Hello from parent!";
                write_pipe.write_all(message.as_bytes()).expect("Failed to write to pipe");

                // Print the message sent to child
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
    ipc_phase(); // Call the IPC function
    println!("Phase IPC Completed.");
}
