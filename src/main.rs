use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct FinanceRecord {
    income: f64,
    expenses: Vec<Expense>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Expense {
    category: String,
    amount: f64,
}

impl FinanceRecord {
    fn new() -> Self {
        FinanceRecord {
            income: 0.0,
            expenses: Vec::new(),
        }
    }

    fn add_income(&mut self, amount: f64) {
        self.income += amount;
    }

    fn add_expense(&mut self, category: &str, amount: f64) {
        self.expenses.push(Expense {
            category: category.to_string(),
            amount,
        });
    }

    fn total_expenses(&self) -> f64 {
        self.expenses.iter().map(|e| e.amount).sum()
    }

    fn net_income(&self) -> f64 {
        self.income - self.total_expenses()
    }
}

fn load_data() -> FinanceRecord {
    let path = "finance_data.json";
    if Path::new(path).exists() {
        let data = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&data).expect("JSON was not well-formatted")
    } else {
        FinanceRecord::new()
    }
}

fn save_data(record: &FinanceRecord) {
    let json = serde_json::to_string(record).expect("Failed to serialize data");
    let mut file = File::create("finance_data.json").expect("Unable to create file");
    file.write_all(json.as_bytes())
        .expect("Unable to write data");
}

fn main() {
    let matches = Command::new("BudgetWise")
        .version("1.0")
        .author("Your Name")
        .about("Track your income and expenses")
        .arg(
            Arg::new("add_income")
                .short('i')
                .long("income")
                .value_name("AMOUNT")
                .help("Add an income amount")
                .required(false),
        )
        .arg(
            Arg::new("add_expense")
                .short('e')
                .long("expense")
                .value_name("CATEGORY,AMOUNT")
                .help("Add an expense in the format 'category,amount'")
                .required(false),
        )
        .get_matches();

    let mut record = load_data();

    if let Some(income_str) = matches.get_one::<String>("add_income") {
        let income: f64 = income_str.parse().expect("Invalid income amount");
        record.add_income(income);
        println!("Added income: {}", income);
    }

    if let Some(expense_str) = matches.get_one::<String>("add_expense") {
        let parts: Vec<&str> = expense_str.split(',').collect();
        if parts.len() == 2 {
            let category = parts[0].to_string();
            let amount: f64 = parts[1].parse().expect("Invalid expense amount");
            record.add_expense(&category, amount);
            println!("Added expense: {} - ${}", category, amount);
        } else {
            println!("Please provide the expense in the format 'category,amount'");
        }
    }

    save_data(&record);

    println!("\n=== Summary ===");
    println!("Total Income: ${:.2}", record.income);
    println!("Total Expenses: ${:.2}", record.total_expenses());
    println!("Net Income: ${:.2}", record.net_income());
}
