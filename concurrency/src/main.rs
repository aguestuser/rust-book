use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    {
        /*****************************************
         * spawning a thread (concurrent execution)
         *****************************************/
        println!(">>> spawning a thread  <<<");

        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from mthe spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        // block on main thread until spawned thread completes
        // (else you'd never see print statements past 5 from spawned thread)
        handle.join().unwrap();
    }

    {
        /*********************************************************************
         * using move closures with threads (for data sharing btw/ threads)
         ********************************************************************/

        let v = vec![1, 2, 3];

        let handle = thread::spawn(move || {
            println!("here's a vector from the main thread: {:?}", v);
        });

        // but we can't reference v in main thread anymore!! this won't compile..
        // drop(v);

        handle.join().unwrap();
    }

    {
        /*******************************
         * channel with single message
         ******************************/
        println!(">>> channel w/ single message  <<<");

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
            // cannot access val anymore here (moved)
        });

        // blocks on main thread until message is received
        // (this is generally a bad idea!)
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    {
        /*******************************
         * channel w/ multiple messages
         ******************************/
        println!(">>> channel w/ multiple messages <<<");

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            // let val = String::from("hi");
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread!"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                // cannot access val anymore here (moved)
                thread::sleep(Duration::from_millis(50));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }

        println!("i won't print until channel is consumed");
    }

    {
        /***********************************
         * channel w/ multiple transmitters
         ***********************************/
        println!(">>> channel w/ multiple transmitters <<<");

        let (tx, rx) = mpsc::channel();
        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            // let val = String::from("hi");
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread!"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                // cannot access val anymore here (moved)
                thread::sleep(Duration::from_millis(50));
            }
        });

        thread::spawn(move || {
            // let val = String::from("hi");
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you!"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                // cannot access val anymore here (moved)
                thread::sleep(Duration::from_millis(50));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }

        println!("i won't print until channel is empty");
    }

    {
        /****************************
         * mutexe in single thread
         *****************************/
        println!(">>> mutex in single thread <<<");

        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
            // as num goes out of scope,
            // we implicitly call `drop` on the mutex
            // and thus release the lock
        }
        println!("m = {:?}", m);
    }

    {
        /*****************************
         * mutex with multiple threads
         *****************************/
        println!(">>> mutex with multiple threads <<<");

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
