pub fn demo() {
    let successful_transaction: Option<Result<&str, &str>> = Some(Ok("Transaction successful!"));
    let failed_transaction: Option<Result<&str, &str>> = Some(Err("Transaction failed due to insufficient funds!"));
    let no_transaction: Option<Result<&str, &str>> = None;

    match successful_transaction {
        Some(Ok(message)) => println!("Success: {}", message),
        Some(Err(message)) => println!("Error: {}", message),
        None => println!("No transaction result available."),
    }
    
    match failed_transaction {
        Some(Ok(message)) => println!("Success: {}", message),
        Some(Err(message)) => println!("Error: {}", message),
        None => println!("No transaction result available."),
    }
    
    match no_transaction {
        Some(Ok(message)) => println!("Success: {}", message),
        Some(Err(message)) => println!("Error: {}", message),
        None => println!("No transaction result available."),
    }
}