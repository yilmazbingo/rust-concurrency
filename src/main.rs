/* 
   to make our async code run concurrently we can use tokio tasks. a task is lighweight non blocking unit of execution

   tasks are scheduled by tokio runtime rather than the os itself, therefore creating new tasks or switching between tasks does not require the involvement of the underlying os.
   the involvement of os is associated with lots of overhead and takes alot of time, for instance to simply swithc between different processes, the os uses the `context swithching`
   which requires saving a process related information of one process and loading processing related information for the other process. the process information is stored in a data structure 
   called process control block which contains information such as the memory address of the next instruction that neets to be executed. 
   the registers data are used by the process list of open files, CPU scheduling, information, memory management, information, IO status and others. tokio tasks do not
   have those overheads, therefore they are "lightweight"

   tasks are not non-blocking. typically when os thread performs IO or must synchronize with another thread, it blocks. thereby allowing the operating system to schedule another
   thread which therefore needs some sort of context switch. When a task cannot contiue execution, it must yield instead, allowing the tokio runtime to schedule another task.
   the underlying os is not informed and therefore costly operations related to context swtiches are being avoided

    the scheduling on a majority of the os is based on the notion of preemption. this means that os allows each thread to run for some predetermined
   period of time and then it preempts it, temporarily pausing the execution and then switching to another thread. this involves the cost of context switching between different processes
   or threads. tasks are scheduled cooperatively means that a task is allowed to run until it yields, indicating to the tokio runtime scheduler that it cannot currently continue
   execution. when a task yields, the tokio runtime switches to execute the next available task. the important difference is that the task itself will yield. 

 */
#[tokio::main]
async fn mainn(){
    let mut handles=vec![];
    println!("this code is not part of the async block");
    let s1=String::from("Huge computation function");
    let s2=String::from("simpler computaion function");
    // with move async block takes the ownership of the variables
    // we really do not need to use move keyword. async also handles the ownership. we use move to pass the ownerhsip properly
    let aw1=tokio::spawn(async move {
        huge_computation(s1).await;
    });
   
    handles.push(aw1);
    let aw2=tokio::spawn(async move {
        simpler_computation(s2).await;
    });
    handles.push(aw2);
    // before exit we make sure all the tasks are bing completed
    for handle in handles{
        handle.await.unwrap();
    }
    println!("All the tasks are completed");
}

async fn huge_computation(s:String){
    println!("{:?} has started",s);
    for i in 0..100_000_000{
    }
    println!("{:?} is now completed",s);
}
// this will be completed first becuase huge_computation will tkae time and it yields i
async fn simpler_computation(s:String){
    println!("{:?} has started",s );
    println!("{:?} is now completed",s);
}

use tokio::select;
#[tokio::main]
async fn main(){
    // select allows us to execute multiple async tasks concurrently and act up on the one executes first. it is useful when you are waiting for multiple things and you are happy if one out of many things happens
    // select macro has to be inside the async code
    // only one of the arms will be executed. select does not yield the code
    select! {
        _=function_1()=>println!("Function 1 is completed first"),
        _=function_2()=>println!("functon 2 is completed first"),
    };
    /*
        select will chose one of the two tasks. if it choses function_1, since the tasks are being managed by the runtime, therefore, when a task is being picked up and executed for some time, the runtime may decide to yield back and therefore the selcet will be given another chance to to select someone else because none of the task has been completed yet. 
     */
    let aw1=tokio::spawn(async move {
        function_1().await;
    });
    let aw2=tokio::spawn(async move{
        function_2().await;
    });

    select! {
        _=aw1=>println!("Function 1 is selected in spawn"),
        _=aw2=>println!("Function 2 is selected in spawn"),
    }
    // tokio::join to make sure both async code run

}

async fn function_1(){
    println!("function 1 has started");
    for i in 0..100_000_000{

    }
    println!("Function 1 has ended");
}

async fn function_2(){
    println!("function 2 has started");
    println!("function 2 has ended");

    for i in 0..100_000_000{

    }
    println!("Function 1 has ended");
}
