// From the Go language doc: "Do not communicate by sharing memory; instead, 
// share memory by communicating".
// Rust provides message-sending concurrency by channel.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc: multiple producer, single consumer.
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // the spawned thread needs to own the transmitting end, so use `move`
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // `recv` block the thread until the message is available. If the sending
    // end is cloed, an Err is returned.
    // `try_recv` doesn't block, and return Result<T, E> immediately.
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // Channels and ownership transference
    // Ownership rules play a vital role in message sending to support fearless
    // concurrency. 
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap(); // `send` takes ownership
            // println!("val is {}", val);
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    // Sending multiple values
    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    // Creat multiple producers by cloning the transmitter
    {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }
}
