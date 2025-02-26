#[test]
fn test_ipc_data_integrity() {
    use os_pipe::pipe;
    use std::io::{Read, Write};
    use libc;

    let (mut read_pipe, mut write_pipe) = pipe().expect("Failed to create pipe");

    unsafe {
        match libc::fork() {
            -1 => panic!("Fork failed"),
            0 => {
                drop(write_pipe);
                let mut buf = String::new();
                read_pipe.read_to_string(&mut buf).unwrap();
                assert_eq!(buf, "Hello from parent!", "Child did not receive the correct message");
                std::process::exit(0);
            }
            _ => {
                drop(read_pipe);
                write_pipe.write_all(b"Hello from parent!").unwrap();
            }
        }
    }
}