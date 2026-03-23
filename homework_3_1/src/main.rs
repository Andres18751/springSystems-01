mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(1000.0);
    println!("Created account with balance: ${:.2}", account.balance());

    account.deposit(500.50);
    println!("After deposit: ${:.2}", account.balance());

    account.withdraw(200.25);
    println!("After withdrawal: ${:.2}", account.balance());

    account.withdraw(2000.0);
    println!("After attempting to overdraw: -${:.2}", account.balance());

    account.deposit(-100.0);
    println!("Failed to deposit negative dollars to account. After trying to deposit negative: ${:.2}", account.balance());

    println!("Final balance: ${:.2}", account.balance());
}