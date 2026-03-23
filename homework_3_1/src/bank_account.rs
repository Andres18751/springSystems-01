#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
         BankAccount {
            balance: initial_balance,
        }
        // Implement this method
    }

    pub fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
        // Implement this method
    }

    pub fn withdraw(&mut self, amount: f64) {
         if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
        // Implement this method
    }

    pub fn balance(&self) -> f64 {
         self.balance
        // Implement this method
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_account() {
        let account = BankAccount::new(100.0);
        assert!(account.balance(), 100.0);
        // Write a test for creating a new account
    }

    #[test]
    fn test_deposit() {
        let mut account = BankAccount::new(50.0);
        account.deposit(25.0);
        assert!(account.balance(), 75.0);
        // Write a test for depositing money
    }

    #[test]
    fn test_withdraw() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(30.0);
        assert!(account.balance(), 70.0);
        // Write a test for withdrawing money
    }

    #[test]
    fn test_withdraw_failed(){
        let mut account = BankAccount::new(50.0);
        account.withdraw(-20.0);
        assert!(account.ba!lance(), 50.0); 
    }

    #[test]
    fn deposting_negative_amount(){
        let mut account = BankAccount::new(50.0);
        account.deposit(-10.0);
        assert!(account.balance(), 50.0);
    }
    // Add more tests here
}