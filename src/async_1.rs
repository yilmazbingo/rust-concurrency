/*
     the park will block the execution of the current thread until it is unparked. it help in synchronization problem
 */

use std::thread;
use std::time::Duration;
// this is code for thread.park
fn mainn(){
    let job_1=thread::spawn(||{
        println!("--job thread has started");
        println!("Waiting for job 2 to complete");
        // this will block the execution until we unparked
        thread::park();
        println!("--job 1 resumed --");
        println!("---job 1 finished");

    });
    let job_2=thread::spawn(||{
        println!("--Job 2 started --");
        println!("-- Job 2 finished -- ");
    });

    job_2.join().unwrap();
    println!("job 2 is not completed");
    println!("job 1 will now resume");
    job_1.thread().unpark();
    job_1.join().unwrap();
}
fn main(){
    let job_1=thread::spawn(||{
        println!("--job thread has started");
        println!("Waiting for job 2 to complete");
        // this will block the execution until we unparked. if we dont call unparked it will wait the specified duration
        thread::park_timeout(Duration::from_secs(2));
        // the differeence is even we call thread().unparked will not work here
        // thread::sleep(Duration::from_secs(2));
        // yield generates a signal from the calling thread. it is willing to give up its remaining time slice so that the os may schedule other threads on the CpU
        // thread::yield_now();
        println!("--job 1 resumed --");
        println!("---job 1 finished");

    });
    let job_2=thread::spawn(||{
        println!("--Job 2 started --");
        println!("-- Job 2 finished -- ");
    });

    job_2.join().unwrap();
    println!("job 2 is not completed");
    println!("job 1 will now resume");
    // job_1.thread().unpark();
    job_1.join().unwrap();
}