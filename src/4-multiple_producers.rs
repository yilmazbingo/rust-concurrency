// multiple-producer, single-consumer."
// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;
use std::{sync::mpsc, thread, time::Duration};
fn mainn(){
    let (tx,rx)=mpsc::channel();
    let t=thread::spawn(move||{
        let my_vec=vec![1,2,3,4,5];
        for i in my_vec{
            tx.send(i).unwrap();
        }
    });
    
    // for received_vals in rx{
    //     println!("I received the value of {}",received_vals);
    // }
    // we could use iter function. we could run both for and this because for loop would consume the iterator
    // rx.iter() returns an iterator over the values received through the receiving end (rx) of the channel. It doesn't block and will keep iterating as values become available.
    /*
    .collect::<Vec<i32>>(): The collect() method is used to consume an iterator and collect its elements into a specified collection type. In this case, it collects the elements from the iterator returned by rx.iter() into a Vec<i32>.

    The ::<Vec<i32>>() part specifies the target collection type (Vec<i32> in this case) that will hold the elements collected from the iterator.
     */
    let receivec_vals_vec=rx.iter().collect::<Vec<i32>>();
    println!("the received values are {:?}",receivec_vals_vec);

}

fn main1(){
    let (tx,rx)=mpsc::channel();
    // we cannot pass tx to multiple threads. that is why we are cloning
    // in general, to send out the data using the same channel, we will need to make a clone copy of the sender
    let tx1=tx.clone();
    thread::spawn(move ||{
        // these values are received in order
        let my_vec=vec![1,2,3,4,5];
        for i in my_vec{
            tx.send(i).unwrap();
            // i want to give a chance to another thread
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move ||{
        let my_vec=vec![6,8,9];
        for i in my_vec{
            tx1.send(i).unwrap();
            // i want to give a chance to another thread
            thread::sleep(Duration::from_secs(1));
        }
    });

    // in main thread we write code to receive all the values
    /* the values sent from same thread are received on the same order. this is the due to fifo nature of the queue
       within the thread, the execution is always sequential and therefore the output is always deterministic, however between the threads, the execution is not sequential and therefore
       the output may be undeterministic
     */
    for received_vals in rx{
        println!("i received the value of {}",received_vals);
    }
}

// use of threads inside the functions

fn timer(d:i32,tx:mpsc::Sender<i32>){
    thread::spawn(move ||{
        println!("{} send!",d);
        tx.send(d).unwrap();
    });
}
fn main(){
    let (tx,rx)=mpsc::channel();
    /*   - since we send the clone  of sender, original sender is always alive
         - if the sender is alive and in scope, then the receiver will keep on waiting since it believes that one of the sender is still alive and he may send out message
         as a result program goes to a waiting state
         - if any of the sender is alive for a specific channel, then the receiver will keep on waiting for the values in the same channel.
    */
    for i in 0..5{
        timer(i,tx.clone())
    }
    // we need to make sure sender is dropped before we receive the values
    drop(tx);

    for receiving_val in rx{
        println!("{} received",receiving_val);
    }
}