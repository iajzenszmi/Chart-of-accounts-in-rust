use std::io;

#[derive(Debug, Clone)]
struct Account {
    number: u32,
    description: String,
}

struct ChartOfAccounts {
    accounts: Vec<Account>,
}

impl ChartOfAccounts {
    fn new() -> Self {
        Self { accounts: Vec::new() }
    }

    fn add_account(&mut self, number: u32, description: String) {
        if self.accounts.len() >= 10 {
            println!("Account table is full.");
        } else if self.accounts.iter().any(|acc| acc.number == number) {
            println!("Account number {} already exists.", number);
        } else {
            self.accounts.push(Account { number, description });
            println!("Account added successfully.");
        }
    }

    fn delete_account(&mut self, number: u32) {
        if let Some(pos) = self.accounts.iter().position(|acc| acc.number == number) {
            self.accounts.remove(pos);
            println!("Account {} deleted successfully.", number);
        } else {
            println!("Account {} not found.", number);
        }
    }

    fn display_accounts(&self) {
        if self.accounts.is_empty() {
            println!("No accounts available.");
        } else {
            println!("Current Accounts:");
            for account in &self.accounts {
                println!("Account Number: {}, Description: {}", account.number, account.description);
            }
        }
    }
}

fn main() {
    let mut chart = ChartOfAccounts::new();

    loop {
        println!("\nChart of Accounts System - Demo");
        println!("1. Add Account");
        println!("2. Delete Account");
        println!("3. Display Accounts");
        println!("4. Exit");
        println!("Enter your option:");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input.");
        let option: u32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 4.");
                continue;
            }
        };

        match option {
            1 => {
                let (number, description) = get_account_details();
                chart.add_account(number, description);
            }
            2 => {
                let number = get_account_number();
                chart.delete_account(number);
            }
            3 => chart.display_accounts(),
            4 => {
                println!("Exiting program.");
                break;
            }
            _ => println!("Invalid option. Please try again."),
        }
    }
}

fn get_account_details() -> (u32, String) {
    println!("Enter Account Number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read input.");
    let number: u32 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid account number. Setting it to 0.");
            0
        }
    };

    println!("Enter Account Description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read input.");

    (number, description.trim().to_string())
}

fn get_account_number() -> u32 {
    println!("Enter Account Number:");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read input.");
    match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid account number. Setting it to 0.");
            0
        }
    }
}
