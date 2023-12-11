/*
    Async code is alternative to threads. due to the yielding schedule, it is sometimes referred to as cooperative scheduling as it gives control back to the runtime. 

    Async returns something which implements future trait which has many details but in an abstract level it has a poll method which can either return a ready state with the return
    value is available or pending

    Rust futures are lazy so main cannot be async. solution is `executer` or `runtime`. the executer or runtime is a program whose job is to take the tompost future, in this case
    it is the main function and manually poll them to completion. it is also responsible for running multiple futures in parallel to make the concurrency possible between
    multiple futures. the standard library does not provide an async runtime so in order to run our async code we need to use community build async runtime, the most popular one is called 
    `tokio`. 

    Laziness has some benefits: the first benefit is futures are zero cost abstractions. this means you want to incur a run time cost unless you actually use the futures. another
    benefit is they are easy to cancel. In order to cancel a particular future, we call the `drop` method on the future which will drop it and thereby not allowing it to be pulled. 
 */

async fn printing(){
    println!("I am async function")
}
// this an attribute macro that allows our main() to be async and specifies that our async code will be executed by the tokio runtime. this drives our main to go to completion
#[tokio::main]
async fn main(){
    // this implements trait Future
     
//  let x=printing().await;  .await can only work inside the async block

    let x=printing().await;

    
}