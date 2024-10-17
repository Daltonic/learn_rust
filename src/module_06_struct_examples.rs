struct BlockchainUser {
    username: String,
    public_key: String,
    balance: u64,
    active: bool,
}

impl BlockchainUser {
    fn new(username: &str, public_key: &str, balance: u64) -> Self {
        Self {
            username: String::from(username),
            public_key: String::from(public_key),
            balance,
            active: true,
        }
    }

    fn display_info(&self) {
        println!(
            "Name: {}, Public Key: {}, Balance: {}, Active: {}",
            self.username, self.public_key, self.balance, self.active
        )
    }

    fn update_public_key(&mut self, public_key: &str) {
        self.public_key = public_key.to_string();
    }

    fn deactive(&mut self) {
        self.active = false;
    }

    fn add_balance(&mut self, amount: u64) {
        self.balance += amount;
        println!("Added {} tokens to user: {}", amount, self.username);
    }
}

struct Transaction(u64, String);

// #[derive(Debug)]
struct Block {
    index: u32,
    miner: BlockchainUser,
    transaction_count: u32,
}

pub fn demo() {
    let mut user1 = BlockchainUser::new("Daltonic", "0x2gvscfshb...", 300);
    user1.display_info();

    user1.update_public_key("A9Z8Y7X6");
    user1.display_info();

    user1.deactive();
    user1.display_info();

    let tx1 = Transaction(500, String::from("0xB2C3D4E5..."));
    let tx2 = Transaction(300, String::from("0xF3G4H5I6..."));

    println!("Transaction 1: {} tokens sent to {}", tx1.0, tx1.1);
    println!("Transaction 2: {} tokens sent to {}", tx2.0, tx2.1);

    let block = Block {
        index: 1,
        miner: user1,
        transaction_count: 2,
    };

    println!(
        "Block Index: {}, Miner: {}, Transactions: {}, Active: {}",
        block.index, block.miner.username, block.transaction_count, block.miner.active
    );
}
