// How to handle data between multiple threads. this is very important topic in concurrency since the threads can execute concurrently. therefore we need to be careful while passing the data between the threads to ensure that reads and updation to the data is always done in the right way. 
// the mechanism which is used for handling, transferring or using shared data between multiple threads is called MESSAGE PASSING. a channel has transmitter and receiver
/*
   - the channel is set to be closed if either the transmitter or the receiver is dropped. channles are like QUEUE, FIFO
*/

use std::thread;
// multiple producer single consumer. allows multiple senders but only one consumer
use std::sync::mpsc;
fn main(){
    // transmitter and receiver. they are owned values not references
    let (tx,rx)=mpsc::channel();
    // this thread will create some data and it will pass it to the main thread. 
    thread::spawn(move || {
        // spawn thread needs to own the tranmistter to be able to send out messages through the channel
        let val=String::from("some data from the sender");
        println!("Value is sending from the thread");
        // we are calling unwrap to panic in case of an error
        
        tx.send(val).unwrap();
        println!("This migh be executed after the statemn in the main");
        // once the value is sent out, it is no more with the thread and therefore its ownership will be transferred to the thread where it has been received. 
        // println!("testing the ownership is gone {}",val) // borrow of moved value: `val`
        // primitives are not moved but rather being copied. if our val was i32, compiler would not have any issues
    });
    // this function blocks the thread which calls it and wait until the value is sent down the channel. in this case it is invoked by the MAIN thread, it will block the main thread
    // let received=rx.recv().unwrap();
    // THE "received" is value that sent
    // println!("Received {}",received)

    // this method is very useful if the thread has other work to do while waiting for the messages
    // non blocking. it will imeediately return the result, if the message is not available it will return an error. we can write a loop if the data is available
    let mut received_status=false;
    // if there are multiple threads, they will get a chance as determined by the scheduling algorithm of the operating system
    while received_status!=true{
        // try_recv is non-blocking
        match rx.try_recv(){
            Ok(received_value)=>{
                println!("Received value in try_recv is {:?}",received_value);
                received_status=true;
            }
            Err(_)=>{println!("I am doing ");}
        }
    }
    // rx.try_recv();

}