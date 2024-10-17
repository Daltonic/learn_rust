use std::time::Duration;
use std::{sync::mpsc, thread};
use tokio::time::{sleep, Duration as TokioDuration};

pub fn basic_thread_example() {
    let handle = thread::spawn(|| {
        for i in 1..4 {
            println!("Blockchain node thread says: {}", i);
            thread::sleep(Duration::from_millis(500));
        }
    });

    for i in 1..4 {
        println!("Main blockchain operation says: {}", i);
        thread::sleep(Duration::from_millis(1500));
    }

    handle.join().unwrap();
}

pub fn thread_with_move_example() {
    let block_data: Vec<i32> = vec![1001, 1002, 1003, 1004, 1005];

    let handle = thread::spawn(move || {
        println!("Node processed block data: {:?}", block_data);
    });

    handle.join().unwrap();
}

pub async fn basic_async_example() {
    async_blockchain_operation().await;
}

async fn async_blockchain_operation() {
    println!("Blockchain async operation started...");
    sleep(TokioDuration::from_secs(2)).await;
    println!("2sec spent...");
    sleep(TokioDuration::from_secs(2)).await;
    println!("Blockchain async operation completed");
}

pub async fn parallel_async_example() {
    let task1 = blockchain_task(1, 2);
    let task2 = blockchain_task(2, 4);

    tokio::join!(task1, task2);

    println!("Both blockchain tasks completed!");
}

async fn blockchain_task(id: u8, duration: u64) {
    println!(
        "Blockchain Task {} started, waiting for {} seconds...",
        id, duration
    );
    sleep(TokioDuration::from_secs(duration)).await;
    println!("Blockchain Task {} completed!", id);
}

pub fn thread_with_channels_example() {
    let (tx, rx) = mpsc::channel::<String>();

    let handle = thread::spawn(move || {
        let messages: Vec<String> = vec![
            String::from("Block 101"),
            String::from("Block 102"),
            String::from("Block 103"),
        ];

        for message in messages {
            println!("Sub blockchain thread sent: {}", message);
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });

    for received in rx {
        println!("Main blockchain thread received: {}", received)
    }

    handle.join().unwrap();
}

pub async fn async_with_return_value_example() {
    let result = async_calculate_block_reward(10, 5).await;
    println!("Result of block reward calculation: {}", result);
}

async fn async_calculate_block_reward(block_time: u32, reward_rate: u32) -> u32 {
    println!("Calculating block reward asynchronously...");
    sleep(TokioDuration::from_secs(1)).await;
    block_time * reward_rate
}

pub fn spawn_multiple_threads_example() {
    let mut handles: Vec<_> = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Blockchain node Thread {} started.", i);
            thread::sleep(Duration::from_millis(1000));
            println!("Blockchain node Thread {} finished.", i);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap()
    }
}

pub async fn demo() {
    basic_thread_example();
    thread_with_move_example();
    basic_async_example().await;
    parallel_async_example().await;
    thread_with_channels_example();
    async_with_return_value_example().await;
    spawn_multiple_threads_example();
}
