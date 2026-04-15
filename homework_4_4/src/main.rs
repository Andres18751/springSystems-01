use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    const NUM_PRODUCERS: usize = 2;
    const NUM_CONSUMERS: usize = 3;

    // TODO: Create a channel for sending numbers
    let (ax, bx) = mpsc::channel();
    let bx = Arc::new(Mutex::new(bx));

    // TODO: Create 2 producer threads
    let mut producer_handles = vec![];
    for id in 0..NUM_PRODUCERS {
        let ax_clone = ax.clone();
        // Each producer produces half of the items (or the remainder for the last one)
        let items_per_producer = ITEM_COUNT / NUM_PRODUCERS;
        producer_handles.push(thread::spawn(move || {
            producer(id, ax_clone, items_per_producer);
        }));
    }
    
    // TODO: Create 3 consumer threads
    let mut consumer_handles = vec![];
    for id in 0..NUM_CONSUMERS {
        let bx_clone = Arc::clone(&bx);
        consumer_handles.push(thread::spawn(move || {
            consumer(id, bx_clone);
        }));
    }
    
    // TODO: Wait for all threads to finish
    for handle in producer_handles {
        handle.join().unwrap();
    }

    for i in 0..NUM_CONSUMERS {
        ax.send(TERMINATION_SIGNAL).unwrap();
    }

    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, ax: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::rng();
    for i in 0..item_count {
        let num = rng.random_range(1..=100);
        println!("Producer {} produced {}", id, num);
        ax.send(num).unwrap();
        thread::sleep(Duration::from_millis(100)); // simulate work
    }
    println!("Producer {} finished.", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, bx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let received = {
            let lock = bx.lock().unwrap();
            lock.recv().unwrap()
        };
        if received == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal. Exiting.", id);
            break;
        }
        println!("Consumer {} processed number {}", id, received);
        thread::sleep(Duration::from_millis(150)); // simulate processing
    }
}