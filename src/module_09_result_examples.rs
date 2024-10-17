pub fn result_basics_example() {
    let success_transaction: Result<u64, &str> = Ok(200);
    let failed_transaction: Result<u64, &str> =
        Err("Transaction failed with due to insufficient funds!");

    match success_transaction {
        Ok(amount) => println!("Transaction succeeded with {} tokens.", amount),
        Err(e) => println!("Transaction failed with error: {}", e),
    }

    match failed_transaction {
        Ok(amount) => println!("Transaction succeeded with {} tokens.", amount),
        Err(e) => println!("Transaction failed with error: {}", e),
    }
}

pub fn unwrap_or_example() {
    let failed_transaction: Result<u64, &str> = Err("Transaction failed!");

    let amount_or_default = failed_transaction.unwrap_or(0);
    println!(
        "Transaction failed, defaulting to: {} tokens",
        amount_or_default
    );
}

pub fn map_err_example() {
    let failed_transaction: Result<u64, &str> = Err("Insufficient gas fees!");

    let transformed_error = failed_transaction.map_err(|e| format!("Blockchain Error: {}", e));
    println!("Transformed error: {:?}", transformed_error);
}

pub fn demo() {
    result_basics_example();
    unwrap_or_example();
    map_err_example();
}
