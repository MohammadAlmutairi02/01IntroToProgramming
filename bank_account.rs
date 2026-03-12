#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    // Create a new bank account
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: if initial_balance >= 0.0 { initial_balance } else { 0.0 },
        }
    }

    // Deposit money
    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    // Withdraw money
    pub fn withdraw(&mut self, amount: f64) {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    // Check balance
    pub fn balance(&self) -> f64 {
        self.balance
    }

    // Bonus: Apply interest
    pub fn apply_interest(&mut self, rate: f64) {
        if rate > 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        let acc = BankAccount::new(100.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        let mut acc = BankAccount::new(100.0);
        acc.deposit(50.0);
        assert!((acc.balance() - 150.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(40.0);
        assert!((acc.balance() - 60.0).abs() < EPSILON);
    }

    #[test]
    fn test_balance() {
        let acc = BankAccount::new(200.0);
        assert!((acc.balance() - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_negative_deposit() {
        let mut acc = BankAccount::new(100.0);
        acc.deposit(-50.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_more_than_balance() {
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(200.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_negative_withdraw() {
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(-20.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_apply_interest() {
        let mut acc = BankAccount::new(100.0);
        acc.apply_interest(0.10);
        assert!((acc.balance() - 110.0).abs() < EPSILON);
    }
}