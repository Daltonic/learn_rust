pub fn basic_if_else() {
    let transaction_amount = 0;

    if transaction_amount > 0 {
        println!("Transaction is valid.");
    } else if transaction_amount < 0 {
        println!("Invalid transaction: Negative amount.");
    } else {
        println!("Transaction amount is zero, no transfer.");
    }
}

pub fn match_example(day: u8) {
    let block_day = match day {
        1 => "Block production on Monday",
        2 => "Validator rewards on Tuesday",
        3 => "Transaction settlement on Wednesday",
        4 => "Governance voting on Thursday",
        5 => "Network upgrade on Friday",
        6 => "Node maintenance on Saturday",
        7 => "No activity on Sunday",
        _ => "Invalid block day",
    };

    println!("Blockchain activity: {}", block_day);
}

pub fn while_loop_example() {
    let mut pending_transactions = 0;

    while pending_transactions < 5 {
        println!(
            "Processing transaction number: {}",
            pending_transactions + 1
        );
        pending_transactions += 1;
    }

    println!("All transactions processed.");
}

pub fn for_loop_example() {
    let staking_rewards = [10, 20, 30, 40, 50];

    for reward in staking_rewards.iter() {
        println!("Validator reward: {}", reward);
    }

    for block in 1..5 {
        println!("Produced block number: {}", block);
    }
}

pub fn infinite_loop_example() {
    let mut attempts = 0;

    loop {
        println!("Checking blockchain state... attempt: {}", attempts + 1);
        attempts += 1;

        if attempts == 3 {
            println!("Breaking the loop after 3 attempts.");
            break;
        }
    }
}

pub fn match_pattern_example(number: i32) {
    match number {
        1 => println!("Executing token transfer."),
        2 | 3 | 4 | 7 => println!("Executing a prime validator operation."),
        10..=19 => println!("Performing governance action between block 10 and 19."),
        _ => println!("Unrecognized operation."),
    }
}

pub fn let_if_example(reputation_score: i32) {
    let reputation_level = if reputation_score >= 90 {
        "High Reputation"
    } else if reputation_score >= 80 {
        "Good Reputation"
    } else if reputation_score >= 70 {
        "Average Reputation"
    } else if reputation_score >= 60 {
        "Low Reputation"
    } else {
        "Poor Reputation"
    };

    println!(
        "Reputation Score: {}, Reputation Level: {}",
        reputation_score, reputation_level
    );
}

pub fn match_return_example(status_code: i32) -> &'static str {
    match status_code {
        200 => "Transaction Successful",
        404 => "Transaction Not Found",
        500 => "Blockchain Error",
        _ => "Unknown Status",
    }
}

pub fn demo() {
    basic_if_else();
    match_example(5);
    while_loop_example();
    for_loop_example();
    infinite_loop_example();
    match_pattern_example(13);
    let_if_example(78);
    let status_message = match_return_example(200);
    println!("Status message: {}", status_message);
}
