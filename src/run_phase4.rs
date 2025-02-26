use libc::{pthread_mutex_destroy, pthread_mutex_init, pthread_mutex_lock, pthread_mutex_t, pthread_mutex_unlock};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn run_phase4() {
    struct BankAccount {
        balance: i32,
    }

    // Initialize accounts inside Arc<Mutex<>>
    let account1 = Arc::new(Mutex::new(BankAccount {
        balance: 100,
    }));

    let account2 = Arc::new(Mutex::new(BankAccount {
        balance: 200,
    }));

    let transfer = |acc1: Arc<Mutex<BankAccount>>, acc2: Arc<Mutex<BankAccount>>, amount: i32| {
        // Lock both Mutexes in a consistent order to avoid deadlock
        let mut acc1_locked = acc1.lock().unwrap(); // Lock account1's Mutex
        let mut acc2_locked = acc2.lock().unwrap(); // Lock account2's Mutex

        acc1_locked.balance -= amount;
        acc2_locked.balance += amount;

        println!(
            "Transferred {} from Account 1 to Account 2. New Balances: {}, {}",
            amount, acc1_locked.balance, acc2_locked.balance
        );
    };

    let acc1 = Arc::clone(&account1);
    let acc2 = Arc::clone(&account2);

    let handle = thread::spawn(move || {
        transfer(acc1, acc2, 50);
    });

    handle.join().unwrap();

    // Access final balances
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