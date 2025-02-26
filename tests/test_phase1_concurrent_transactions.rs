#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_phase1_concurrent_transactions() {
        struct BankAccount {
            balance: i32,
        }

        // Setup: Create shared account using Arc and Mutex
        let account = Arc::new(Mutex::new(BankAccount { balance: 100 }));
        let mut handles = vec![];

        // Simulate concurrent deposits
        for _ in 0..3 {
            let account_clone = Arc::clone(&account);
            handles.push(thread::spawn(move || {
                let mut acc = account_clone.lock().unwrap();
                acc.balance += 50;
            }));
        }

        // Simulate concurrent withdrawals
        for _ in 0..3 {
            let account_clone = Arc::clone(&account);
            handles.push(thread::spawn(move || {
                let mut acc = account_clone.lock().unwrap();
                acc.balance -= 20;
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let final_balance = account.lock().unwrap().balance;
        assert_eq!(final_balance, 220, "Final balance should be 220");
    }
}