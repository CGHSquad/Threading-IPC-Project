use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn run_phase4() {
    // Struct representing a simple bank account with a balance
    struct BankAccount {
        balance: i32,
    }

    // Initialize two bank accounts inside Arc<Mutex<>> for thread-safe access
    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    let transfer = |acc1: Arc<Mutex<BankAccount>>, acc2: Arc<Mutex<BankAccount>>, amount: i32| {
        let start_time = Instant::now(); // Record the start time for timeout tracking
        let timeout = Duration::from_secs(3); // Set timeout duration (3 seconds)

        loop {
            // Attempt to acquire locks without blocking indefinitely
            let acc1_locked = acc1.try_lock();
            let acc2_locked = acc2.try_lock();

            // If both locks are acquired, perform the transfer
            if let (Ok(mut acc1_guard), Ok(mut acc2_guard)) = (acc1_locked, acc2_locked) {
                acc1_guard.balance -= amount; // Deduct amount from sender
                acc2_guard.balance += amount; // Add amount to receiver

                println!(
                    "Transferred {} from Account 1 to Account 2. New Balances: {}, {}",
                    amount, acc1_guard.balance, acc2_guard.balance
                );
                return; // Exit function after successful transfer
            }

            // If timeout is reached, abort transfer to prevent deadlock
            if start_time.elapsed() >= timeout {
                println!("Deadlock detected! Aborting transfer.");
                return;
            }

            // Short sleep to reduce CPU usage while waiting for lock availability
            thread::sleep(Duration::from_millis(10));
        }
    };

    // Clone Arc references to pass into the new thread
    let acc1 = Arc::clone(&account1);
    let acc2 = Arc::clone(&account2);

    // Spawn a new thread to perform the transfer
    let handle = thread::spawn(move || {
        transfer(acc1, acc2, 50);
    });

    // Wait for the thread to finish execution
    handle.join().unwrap();

    // Retrieve and print the final balances of both accounts
    let acc1_balance = account1.lock().unwrap().balance;
    let acc2_balance = account2.lock().unwrap().balance;
    println!(
        "Final Balances: Account 1 = {}, Account 2 = {}",
        acc1_balance, acc2_balance
    );
}

pub(crate) fn run() {
    println!("Starting Phase 4...");
    run_phase4();
    println!("Phase 4 Completed.");
}
