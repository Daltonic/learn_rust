enum TransactionType {
    Transfer,
    Mint,
    Burn,
    Stake,
}

enum ContractEvent {
    ContractDeployed,
    ContractTerminated,
    TokenTransfer {
        from: String,
        to: String,
        amount: u64,
    },
    OracleUpdate {
        price: f64,
    },
}

enum TransactionStatus {
    Pending,
    Confirmed(u32),
    Failed(String),
}

impl TransactionStatus {
    fn display_status(&self) {
        match self {
            TransactionStatus::Pending => println!("Transaction is currently pending."),
            TransactionStatus::Confirmed(block) => {
                println!("Transaction confirmed in block: {}", block)
            }
            TransactionStatus::Failed(error) => println!("Transaction failed due to: {}", error),
        }
    }
}

enum ContractLifecycle {
    Initialization,
    Active { participants: u32 },
    Paused,
    Terminated,
}

impl ContractLifecycle {
    fn display_state(&self) {
        match self {
            ContractLifecycle::Initialization => println!("Smart contract is being initialized."),
            ContractLifecycle::Active { participants } => {
                println!("Contract is active with {} perticipants.", participants)
            }
            ContractLifecycle::Paused => println!("Contract is currently paused."),
            ContractLifecycle::Terminated => println!("Contract has been terminated."),
        }
    }
}

pub fn demo() {
    let tx_type = TransactionType::Transfer;
    match tx_type {
        TransactionType::Transfer => println!("Processing a token transfer."),
        TransactionType::Mint => println!("Minting new tokens."),
        TransactionType::Burn => println!("Burning tokens from supply."),
        TransactionType::Stake => println!("Staking tokens for rewards."),
    }

    let transfer_event = ContractEvent::TokenTransfer {
        from: String::from("0xEWFEGHN..."),
        to: String::from("0xHJNBEGX..."),
        amount: 500,
    };

    let oracle_event = ContractEvent::OracleUpdate { price: 1234.56 };

    match transfer_event {
        ContractEvent::ContractDeployed => println!("Smart contract deployed!"),
        ContractEvent::ContractTerminated => println!("Smart contract terminated."),
        ContractEvent::TokenTransfer { from, to, amount } => {
            println!("Transfer of {} tokens from {} to {}.", amount, from, to)
        }
        ContractEvent::OracleUpdate { price } => println!("Oracle updated price to: ${}", price),
    }

    match oracle_event {
        ContractEvent::OracleUpdate { price } => println!("New price from oracle: ${}", price),
        _ => println!("Unhandled event."),
    }

    let tx1 = TransactionStatus::Pending;
    let tx2 = TransactionStatus::Confirmed(12345);
    let tx3 = TransactionStatus::Failed("Invalid signature".to_string());

    tx1.display_status();
    tx2.display_status();
    tx3.display_status();

    let contract = ContractLifecycle::Active { participants: 100 };
    let paused_contract = ContractLifecycle::Paused;
    let terminated_contract = ContractLifecycle::Terminated;

    contract.display_state();
    paused_contract.display_state();
    terminated_contract.display_state();
}
