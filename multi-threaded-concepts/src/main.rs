use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// fn main() {
//     let example = thread::spawn(move || {
//         println!("Hello from your first thread")
//     });
//     let _ = example.join();
// }

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("Hello {} from the first thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });

//     for i in 1..5 {
//         println!("Hello {} from the second thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//     for i in 1..4 {
//         println!("Hello {} from the third thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap();
// }

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("first"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("We"),
            String::from("see"),
            String::from("how"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
            String::from("works"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Get message {}", received);
    }
}
