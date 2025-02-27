use std::sync::{Arc, Mutex};
use std::thread;
use os_pipe::pipe;
use std::io::{Read, Write};
use libc;

// ======================== IPC TESTING ========================

#[test]
fn test_ipc_data_integrity() {
    // Create a pipe for interprocess communication
    let (mut read_pipe, mut write_pipe) = pipe().expect("Failed to create pipe");

    unsafe {
        match libc::fork() {
            -1 => panic!("Fork failed"),
            0 => { // Child process
                drop(write_pipe); // Close write end in child
                let mut buf = String::new();
                read_pipe.read_to_string(&mut buf).unwrap();
                // Verify that the message received matches the expected value
                assert_eq!(buf, "Hello from parent!", "Child did not receive the correct message");
                std::process::exit(0);
            }
            _ => { // Parent process
                drop(read_pipe); // Close read end in parent
                write_pipe.write_all(b"Hello from parent!").unwrap();
            }
        }
    }
}

// ======================== THREADING TESTS ========================

#[test]
fn test_phase1_concurrent_transactions() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    // BankAccount structure with balance
    struct BankAccount {
        balance: i32,
    }

    // Shared account with initial balance
    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let mut handles = vec![];

    // Simulate concurrent deposits
    for _ in 0..3 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 50; // Deposit operation
        }));
    }

    // Simulate concurrent withdrawals
    for _ in 0..3 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance -= 20; // Withdrawal operation
        }));
    }

    // Join all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Final balance check
    let final_balance = account.lock().unwrap().balance;
    let expected_balance = 100 + (3 * 50) - (3 * 20); // 100 + 150 - 60 = 190
    assert_eq!(final_balance, expected_balance, "Final balance should be 190");
}


#[test]
fn test_phase2_protect_shared_resource() {
    struct BankAccount {
        balance: i32,
    }

    // Create a shared bank account
    let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let mut handles = vec![];

    // Spawn 10 threads to increment the balance concurrently
    for _ in 0..10 {
        let account_clone = Arc::clone(&account);
        handles.push(thread::spawn(move || {
            let mut acc = account_clone.lock().unwrap();
            acc.balance += 10;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Verify the final balance
    let final_balance = account.lock().unwrap().balance;
    assert_eq!(final_balance, 200, "Final balance should be 200");
}

#[test]
#[should_panic] // Expected to fail due to deadlock
fn test_phase3_deadlock_scenario() {
    struct BankAccount {
        balance: i32,
    }

    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    // First thread locks account1 first, then tries to lock account2
    let t1 = thread::spawn(move || {
        let _lock1 = acc1_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(2));
        let _lock2 = acc2_clone.lock().unwrap(); // Deadlock occurs here
    });

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    // Second thread locks account2 first, then tries to lock account1
    let t2 = thread::spawn(move || {
        let _lock2 = acc2_clone.lock().unwrap();
        thread::sleep(std::time::Duration::from_secs(2));
        let _lock1 = acc1_clone.lock().unwrap(); // Deadlock occurs here
    });

    let _ = t1.join();
    let _ = t2.join();
}

#[test]
fn test_phase4_deadlock_prevention() {
    struct BankAccount {
        balance: i32,
    }

    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    // Function to transfer money between accounts while preventing deadlocks
    let transfer = |acc1: Arc<Mutex<BankAccount>>, acc2: Arc<Mutex<BankAccount>>, amount: i32| {
        // Lock resources in a consistent order to prevent deadlocks
        let mut a1 = acc1.lock().unwrap();
        let mut a2 = acc2.lock().unwrap();
        a1.balance -= amount;
        a2.balance += amount;
    };

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    let t1 = thread::spawn(move || {
        transfer(acc1_clone, acc2_clone, 50);
    });

    t1.join().unwrap();

    // Verify final balances
    assert_eq!(account1.lock().unwrap().balance, 50, "Account 1 should have 50 after transfer");
    assert_eq!(account2.lock().unwrap().balance, 250, "Account 2 should have 250 after transfer");
}
