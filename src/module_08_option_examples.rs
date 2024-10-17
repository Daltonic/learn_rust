pub fn option_basics_example() {
    let user_balance: Option<u64> = Some(1000);
    let user_address: Option<&str> = Some("0xABC123...");
    let no_balance: Option<u64> = None;

    match user_balance {
        Some(balance) => println!("User has a balance of {} tokens.", balance),
        None => println!("No balance found for this account."),
    }

    match user_address {
        Some(address) => println!("Found blockchain address {}.", address),
        None => println!("No address found."),
    }
}

pub fn if_let_example() {
    let user_balance: Option<u64> = Some(1000);

    if let Some(balance) = user_balance {
        println!("Using `if let`: User has a balance of {} tokens.", balance);
    } else {
        println!("No balance found for the user.");
    }
}

pub fn unwrap_example() {
    let user_balance: Option<u64> = Some(1000);

    let balance = user_balance.unwrap();
    println!("Unwrapped balance: {}", balance);
}

pub fn unwrap_or_example() {
    let no_balance: Option<u64> = None;

    let balance = no_balance.unwrap_or(0);
    println!("No balance found, so using a default: {} tokens", balance);
}

pub fn map_example() {
    let some_fee: Option<u64> = Some(10);

    let result = some_fee.map(|fee| fee * 2);
    println!("Doubled tranction fee: {:?}", result);
}

pub fn demo() {
    option_basics_example();
    if_let_example();
    unwrap_example();
    unwrap_or_example();
    map_example();
}
