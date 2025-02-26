use std::sync::{Arc, Mutex};

pub fn run_phase3() {
    struct BankAccount {
        balance: i32,
    }

    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    let thread1 = std::thread::spawn(move || {
        let _lock1 = acc1_clone.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let _lock2 = acc2_clone.lock().unwrap();
        println!("Thread 1: Transferred funds from Account 1 to Account 2");
    });

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    let thread2 = std::thread::spawn(move || {
        let _lock2 = acc2_clone.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let _lock1 = acc1_clone.lock().unwrap();
        println!("Thread 2: Transferred funds from Account 2 to Account 1");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}