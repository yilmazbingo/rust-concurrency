/*
   Web scraping refers to the extraction of data from a website. we will consider grabbing the textual information from many websites and then storing them in a var.
 */

use std::sync::{mpsc,Arc,Mutex};
use std::time::{Duration,Instant};
use std::thread;
// simple and safe http client
use ureq::{Agent,AgentBuilder};

fn main()->Result<(),ureq::Error>{
    let webpages=vec!["https://github.com/accimeesterlin/tpcard-display/blob/main/app.js",
                                 "https://github.com/yilmazbingo/portfolio-nodeServer-typescript/blob/master/src/AppRouter.ts",
                                  "https://github.com/yilmazbingo/portfolio-nodeServer-typescript/blob/master/src/middlewares/errorHandler.ts"];
   // Agent builder is used to accumulate options towards building an agent 
   let agent=ureq::AgentBuilder::new()
   .build();

    let now=Instant::now();
    for web_page in &webpages{
        let web_body=agent.get(web_page).call()?.into_string()?;
        println!("yilma")

    }
    // the amount time elapsed since the instant indicated by the variable now was created
    // {:.2?} fraction part should only contain a couple of digits
    println!("Time taken without threads :{:.2?}",now.elapsed());
    let now=Instant::now();
    let agent=Arc::new(agent);
    let mut handles:Vec<thread::JoinHandle<Result<(),ureq::Error>>>=Vec::new();

    for web_page in webpages{
        println!("yilm");
        let agent_thread=agent.clone();
        let t=thread::spawn(move ||{
            let web_body=agent_thread.get(web_page).call()?.into_string()?;
            Ok(())
        });
        handles.push(t)
    }
    for handle in handles{
        handle.join().unwrap();
    }
    println!("Time take using threds :{:.2?}",now.elapsed());
    Ok(())
}