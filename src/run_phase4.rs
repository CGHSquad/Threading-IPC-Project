use std::sync::{Arc, Mutex};

pub fn run_phase4() {
    struct BankAccount {
        balance: i32,
    }

    let account1 = Arc::new(Mutex::new(BankAccount { balance: 100 }));
    let account2 = Arc::new(Mutex::new(BankAccount { balance: 200 }));

    let transfer = |acc1: Arc<Mutex<BankAccount>>, acc2: Arc<Mutex<BankAccount>>, amount: i32| {
        let mut a1 = acc1.lock().unwrap();
        let mut a2 = acc2.lock().unwrap();
        a1.balance -= amount;
        a2.balance += amount;
        println!(
            "Transferred {} from Account 1 to Account 2. New Balances: {}, {}",
            amount, a1.balance, a2.balance
        );
    };

    let acc1_clone = Arc::clone(&account1);
    let acc2_clone = Arc::clone(&account2);

    let thread = std::thread::spawn(move || {
        transfer(acc1_clone, acc2_clone, 50);
    });

    thread.join().unwrap();
}