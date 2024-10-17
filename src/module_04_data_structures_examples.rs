use std::collections::HashMap;

pub fn array_example() {
    let block_hashes: [u64; 5] = [123456, 789012, 345678, 901234, 567890];
    println!("Block Hashes Array: {:?}", block_hashes);
    println!("First block hash: {}", block_hashes[0]);
    println!("Last block hash: {}", block_hashes[block_hashes.len() - 1]);

    let slice = &block_hashes[1..4];
    println!("Sliced Block Hashes: {:?}", slice);
}

pub fn vector_example() {
    let mut transaction_ids: Vec<&str> = vec!["tx1", "tx2", "tx3"];
    transaction_ids.push("tx4");
    transaction_ids.push("tx5");
    println!("After Adding Transactions: {:?}", transaction_ids);

    transaction_ids.pop();
    println!("After Removing Last Transaction: {:?}", transaction_ids);

    for tx in &transaction_ids {
        println!("Transaction ID: {}", tx);
    }

    if let Some(first_tx) = transaction_ids.get(0) {
        println!("First transaction ID: {}", first_tx);
    }
}

pub fn tuple_example() {
    let user_info: (&str, i32, f64) = ("Alice", 30, 2.5);
    println!("User Info Tuple: {:?}", user_info);

    println!("Username: {}", user_info.0);
    println!("Age: {}", user_info.1);
    println!("Token Balance: {}", user_info.2);

    let (name, age, balance) = user_info;
    println!(
        "Destructured Info: Name = {}, Age = {}, Token Balance = {}",
        name, age, balance
    );
}

pub fn hash_map_example() {
    let mut balanceOf: HashMap<&str, u32> = HashMap::new();

    balanceOf.insert("1C4PHWb8fKdjt2xvdSzNJSUwhVM8B4xekENGnz8p5j6ai", 100);
    balanceOf.insert("1FqQ7LBSfAaRRSFer8c3QFo9rT4AWpKnPGVadApfTWb2E", 150);
    balanceOf.insert("13k6iGXSJamjtKV59r9e4VKkNbpHyiAb7mtVFon9pd61Z", 200);

    println!("User Balances HashMap: {:#?}", balanceOf);

    match balanceOf.get("1FqQ7LBSfAaRRSFer8c3QFo9rT4AWpKnPGVadApfTWb2E") {
        Some(balance) => println!("User's Balance: {}", balance),
        None => println!("No balance found for this user."),
    }

    for (user, balance) in &balanceOf {
        println!("{}'s Balance: {}", user, balance);
    }

    balanceOf
        .entry("1C4PHWb8fKdjt2xvdSzNJSUwhVM8B4xekENGnz8p5j6ai")
        .and_modify(|balance| *balance -= 50);
    println!("Updated User's Balance: {:#?}", balanceOf);

    balanceOf.remove("13k6iGXSJamjtKV59r9e4VKkNbpHyiAb7mtVFon9pd61Z");
    println!("After Removing User: {:#?}", balanceOf);
}

pub fn nested_data_structures_example() {
    let mut user_transactions: HashMap<&str, Vec<&str>> = HashMap::new();

    user_transactions.insert(
        "1C4PHWb8fKdjt2xvdSzNJSUwhVM8B4xekENGnz8p5j6ai",
        vec!["tx1", "tx2", "tx3"],
    );
    user_transactions.insert(
        "1FqQ7LBSfAaRRSFer8c3QFo9rT4AWpKnPGVadApfTWb2E",
        vec!["tx4", "tx5"],
    );

    println!("Users Transactions HashMap: {:#?}", user_transactions);

    if let Some(transactions) =
        user_transactions.get("1FqQ7LBSfAaRRSFer8c3QFo9rT4AWpKnPGVadApfTWb2E")
    {
        println!("User's Transactions: {:#?}", transactions);
    }

    if let Some(transactions) =
        user_transactions.get_mut("1FqQ7LBSfAaRRSFer8c3QFo9rT4AWpKnPGVadApfTWb2E")
    {
        transactions.push("tx6");
    }
    println!("Users Transactions HashMap: {:#?}", user_transactions);
}

pub fn demo() {
    array_example();
    vector_example();
    tuple_example();
    hash_map_example();
    nested_data_structures_example();
}
