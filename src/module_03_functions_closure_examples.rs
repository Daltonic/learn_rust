pub fn basic_function_example() {
    fn add_gas_fee(base_fee: i32, gas_used: i32) -> i32 {
        base_fee + gas_used
    }

    let total_fee = add_gas_fee(100, 50);
    println!("Total gas fee (base fee + gas used): {}", total_fee);
}

pub fn multiple_return_example() {
    fn calculate_transaction(x: i32, y: i32) -> (i32, i32, i32) {
        let total_tokens = x + y;
        let gas_fee = x * y;
        let balance_after_tx = x - y;
        (total_tokens, gas_fee, balance_after_tx)
    }

    let (tokens, fee, balance) = calculate_transaction(8, 3);
    println!(
        "Total Tokens: {}, Gas Fee: {}, Balance after Transaction: {}",
        tokens, fee, balance
    );
}

pub fn higher_order_function_example() {
    fn apply_fee<F>(f: F, gas_limit: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(gas_limit)
    }

    fn calculate_fee(gas_limit: i32) -> i32 {
        gas_limit * 2
    }

    let total_fee = apply_fee(calculate_fee, 100);
    println!("Total fee based on gas limit: {}", total_fee);
}

pub fn basic_closure_example() {
    let transfer = |token: i32, fee: i32| -> i32 { token - fee };
    let final_balance = transfer(100, 5);
    println!("Final balance after transfer: {}", final_balance);
}

pub fn closure_capture_example() {
    let base_gas_price = 10;
    let calculate_total_cost = |gas_used: i32| gas_used * base_gas_price;

    let total_cost = calculate_total_cost(5);
    println!(
        "Total transaction cost: {} (gas used * base gas price)",
        total_cost
    );
}

pub fn returning_closure_example() {
    fn create_staking_reward(muliplier: i32) -> impl Fn(i32) -> i32 {
        move |tokens_staked| tokens_staked * muliplier
    }

    let reward_fn = create_staking_reward(2);
    let staking_reward = reward_fn(500);
    println!("Staking reward for 500 tokens: {}", staking_reward);
}

pub fn closure_ownership_example() {
    let contract_message = String::from("Smart contract executed.");

    let log_message = move || {
        println!("{}", contract_message);
    };

    log_message();
    // print!("{}", contract_message);
}

pub fn demo() {
    basic_function_example();
    multiple_return_example();
    higher_order_function_example();
    basic_closure_example();
    closure_capture_example();
    returning_closure_example();
    closure_ownership_example();
}
