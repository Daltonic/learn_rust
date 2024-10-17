pub fn ownership_example() {
    let token_owner: String = String::from("Alice"); // Store in memory location called "4th Avenue"
    let new_owner: String = token_owner; // Is refering to a value stored in "4th Avenue"
                                         // let new_owner: String = String::from("Alice"); // Store in memory location called "6th Avenue"
    println!("\nNew Token Owner: {}", new_owner);
    // println!("\nOld Token Owner: {}", token_owner);
}

pub fn cloning_example() {
    let transaction_id = String::from("tx123"); // Store in memory location called "4th Avenue"
    let transaction_copy = transaction_id.clone(); // Store in memory location called "6th Avenue"
    println!("Original Transaction ID: {}", transaction_id); // Both are valid
    println!("Cloned Transaction ID: {}", transaction_copy);
}

pub fn copy_trait_example() {
    let token_amount = 100;
    let token_copy = token_amount;

    println!(
        "\nOriginal Token Amount: {}, Copied Token Amount: {}",
        token_amount, token_copy
    );
}

pub fn borrowing_example() {
    let contract_state = String::from("Contract Active");
    print_borrow(&contract_state);
    println!("\nAfter borrowing, Contract State: {}", contract_state);
}

pub fn mutable_borrowing_example() {
    let mut contract_state_mutable = String::from("Contract Pending"); // Store in memory location called "4th Avenue"
    modify_state(&mut contract_state_mutable);
    println!(
        "\nAfter modifying, Contract State: {}",
        contract_state_mutable
    );
}

pub fn print_borrow(contract: &String) {
    println!("\nBorrowed Contract State: {}", contract);
}

pub fn modify_state(contract: &mut String) {
    contract.push_str(" and now active!"); // Modification done at "4th Avenue"
}

// pub fn dangling_reference_example() {
//     let invalid_reference = invalid_borrow();
//     println!("{}", invalid_reference);
// }

// fn invalid_borrow() -> &String {
//     let s: String = String::from("I will be dropped");
//     &s
// }

pub fn demo() {
    ownership_example();
    cloning_example();
    copy_trait_example();
    borrowing_example();
    mutable_borrowing_example();
}
