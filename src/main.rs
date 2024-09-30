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

    fn remove_expense(&mut self, category: &str) {
        self.expenses.retain(|e| e.category != category);
    }

    fn total_expenses(&self) -> f64 {
        self.expenses.iter().map(|e| e.amount).sum()
    }

    fn net_income(&self) -> f64 {
        self.income - self.total_expenses()
    }

    fn list_expenses(&self) {
        if self.expenses.is_empty() {
            println!("No expenses recorded.");
            return;
        }
        println!("=== Expenses ===");
        for expense in &self.expenses {
            println!("{} - ${:.2}", expense.category, expense.amount);
        }
    }

    fn clear_data(&mut self) {
        self.income = 0.0;
        self.expenses.clear();
    }

    fn summary_by_category(&self) {
        let mut category_summary = std::collections::HashMap::new();
        for expense in &self.expenses {
            *category_summary.entry(&expense.category).or_insert(0.0) += expense.amount;
        }

        println!("=== Summary by Category ===");
        for (category, total) in category_summary {
            println!("{} - ${:.2}", category, total);
        }
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
    file.write_all(json.as_bytes()).expect("Unable to write data");
}

fn main() {
    let matches = Command::new("BudgetWise")
        .version("1.0")
        .author("Your Name")
        .about("Track your income and expenses")
        .arg(Arg::new("add_income")
            .short('i')
            .long("income")
            .value_name("AMOUNT")
            .help("Add an income amount")
            .required(false))
        .arg(Arg::new("add_expense")
            .short('e')
            .long("expense")
            .value_name("CATEGORY,AMOUNT")
            .help("Add an expense in the format 'category,amount'")
            .required(false))
        .arg(Arg::new("remove_expense")
            .short('r')
            .long("remove")
            .value_name("CATEGORY")
            .help("Remove an expense by category")
            .required(false))
        .arg(Arg::new("list_expenses")
            .short('l')
            .long("list")
            .help("List all recorded expenses")
            .required(false))
        .arg(Arg::new("clear")
            .short('c')
            .long("clear")
            .help("Clear all data")
            .required(false))
        .arg(Arg::new("summary")
            .short('s')
            .long("summary")
            .help("Show summary of expenses by category")
            .required(false))
        .get_matches();

    let mut record = load_data();

    if let Some(income_str) = matches.get_one::<String>("add_income") {
        let income: f64 = income_str.parse().expect("Invalid income amount");
        record.add_income(income);
        println!("Added income: ${:.2}", income);
    }

    if let Some(expense_str) = matches.get_one::<String>("add_expense") {
        let parts: Vec<&str> = expense_str.split(',').collect();
        if parts.len() == 2 {
            let category = parts[0]; // Use a string slice
            let amount: f64 = parts[1].parse().expect("Invalid expense amount");
            record.add_expense(category, amount); // Pass as a reference
            println!("Added expense: {} - ${:.2}", category, amount);
        } else {
            println!("Please provide the expense in the format 'category,amount'");
        }
    }

    if let Some(category) = matches.get_one::<String>("remove_expense") {
        record.remove_expense(category);
        println!("Removed all expenses in category: {}", category);
    }

    if matches.contains_id("list_expenses") {
        record.list_expenses();
    }

    if matches.contains_id("clear") {
        record.clear_data();
        println!("All data cleared.");
    }

    if matches.contains_id("summary") {
        record.summary_by_category();
    }

    save_data(&record);

    println!("\n=== Summary ===");
    println!("Total Income: ${:.2}", record.income);
    println!("Total Expenses: ${:.2}", record.total_expenses());
    println!("Net Income: ${:.2}", record.net_income());
}
