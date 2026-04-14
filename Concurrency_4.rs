use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define termination signal
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    const ITEM_COUNT: usize = 20;

    // Create channel
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // 🔹 Create 2 producer threads
    for id in 0..2 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / 2);
        });
        handles.push(handle);
    }

    // 🔹 Create 3 consumer threads
    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            consumer(id, rx_clone);
        });
        handles.push(handle);
    }

    // 🔹 Give producers time to finish (simple approach)
    thread::sleep(Duration::from_secs(3));

    // 🔥 Send termination signals
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // 🔹 Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}

// 🔹 Producer (NO rand used)
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    for i in 0..item_count {
        // Simple pseudo-random-like number (no external crate)
        let num = ((id * 100 + i * 37 + 13) % 100) as i32;

        println!("Producer {} produced: {}", id, num);

        tx.send(num).unwrap();

        thread::sleep(Duration::from_millis(200));
    }

    println!("Producer {} finished producing.", id);
}

// 🔹 Consumer
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let received = rx.lock().unwrap().recv().unwrap();

        if received == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal. Exiting.", id);
            break;
        }

        println!("Consumer {} processing: {}", id, received);

        thread::sleep(Duration::from_millis(300));
    }
}