pub trait Describable {
    fn describe(&self) -> String;
}

pub struct Account {
    pub username: String,
    pub balance: u64,
}

impl Describable for Account {
    fn describe(&self) -> String {
        format!("Account: {}, Balance: {}", self.username, self.balance)
    }
}

pub struct Transaction {
    pub id: String,
    pub amount: u64,
    pub sender: String,
    pub receiver: String,
}

impl Describable for Transaction {
    fn describe(&self) -> String {
        format!(
            "Transaction ID: {}, Amount: {} tokens, From: {}, To: {}",
            self.id, self.amount, self.sender, self.receiver
        )
    }
}

pub fn print_description<T: Describable>(trait_item: &T) {
    println!("Description: {}", trait_item.describe());
}

pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Further details hidden...)") //default
    }
}

impl Summarizable for Account {
    fn summary(&self) -> String {
        format!("User: {}, Balance: {} tokens", self.username, self.balance)
    }
}

impl Summarizable for Transaction {}

pub fn multiple_trait_example<T: Describable + Summarizable>(trait_item: &T) {
    println!("Full Description: {}", trait_item.describe());
    println!("Summary: {}", trait_item.summary());
}

pub fn impl_trait_example(trait_item: impl Describable) {
    println!("Impl Trait Description: {}", trait_item.describe())
}

pub fn demo() {
    let account = Account {
        username: String::from("Alice"),
        balance: 5000,
    };

    let transaction = Transaction {
        id: String::from("tx123456789"),
        amount: 150, // 150 tokens
        sender: String::from("Alice"),
        receiver: String::from("Bob"),
    };

    print_description(&account);
    print_description(&transaction);

    println!("Account Summary: {}", account.summary());
    println!("Transaction Summary: {}", transaction.summary());

    multiple_trait_example(&account);
    multiple_trait_example(&transaction);

    impl_trait_example(account);
}
