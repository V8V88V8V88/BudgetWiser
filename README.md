# BudgetWise

BudgetWise is a simple command-line application written in Rust that helps you track your income and expenses. It allows you to add income and expenses, view a summary of your financial status, and stores your data in a JSON file.

## Features

- Add income amounts.
- Add expenses categorized by type.
- Calculate total expenses and net income.
- Data is saved in a JSON file for persistence.

## Requirements

- Rust (1.60.0 or higher)
- Cargo (comes with Rust)

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/BudgetWise.git
   cd BudgetWise
   ```

2. Build the project:

   ```bash
   cargo build
   ```

## Usage

You can run BudgetWise with the following commands:

### Add Income

To add an income amount, use the `--income` or `-i` flag:

```bash
cargo run -- --income <AMOUNT>
```

Example:

```bash
cargo run -- --income 1500
```

### Add Expense

To add an expense, use the `--expense` or `-e` flag followed by the category and amount in the format `category,amount`:

```bash
cargo run -- --expense "<CATEGORY>,<AMOUNT>"
```

Example:

```bash
cargo run -- --expense "groceries,200"
```

### View Summary

After adding income and expenses, the application will display a summary of your total income, total expenses, and net income.

## Data Storage

The application stores financial records in a file named `finance_data.json`. This file is created in the same directory as the executable and persists your data between sessions.

## Contributing

Feel free to submit issues or pull requests. Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Vaibhav
