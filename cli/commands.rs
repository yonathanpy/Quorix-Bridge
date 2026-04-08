use crate::wallet::Wallet;

pub fn deposit(wallet: &mut Wallet, amount: f64) {
    wallet.deposit(amount);
    println!("Deposited: {}", amount);
}

pub fn withdraw(wallet: &mut Wallet, amount: f64) {
    wallet.withdraw(amount);
    println!("Withdrawn: {}", amount);
}

pub fn show_balance(wallet: &Wallet) {
    println!("Current Balance: {}", wallet.balance);
}
