use std::sync::{mpsc, Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver};

use tonic::{transport::Server, Request, Response, Status};

use redis_module::{Context, RedisError, RedisResult, ThreadSafeContext, DetachedFromClient};

use funcloc::func_loc_server::{FuncLoc, FuncLocServer};
use funcloc::{InvokeRequest, InvokeReply};
use std::thread;

use std::time::Duration;

use super::executor::Executor;
use std::rc::Rc;
use crate::{Invoke};
use super::ext::init;

pub mod funcloc {
    tonic::include_proto!("funcloc");
}

// #[derive(Default)]
pub struct MyGreeter {
    exec: Arc<Executor>,
}

impl MyGreeter {
    pub fn new(exec: Arc<Executor>) -> MyGreeter {
        MyGreeter{
            exec,
        }
    }
}

#[tonic::async_trait]
impl FuncLoc for MyGreeter {
    async fn invoke(
        &self,
        request: Request<InvokeRequest>,
    ) -> Result<Response<InvokeReply>, Status> {
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        let inv = Invoke{tx: Mutex::new(tx), req: String::from("hello")};
        self.exec.add_task(inv);

        // self.tx.lock().unwrap().send(Invoke{tx, req: String::from("hello")});

        let res = rx.recv().unwrap();
        println!("{}", res);

        // let res = rx.recv();
        // if !res.is_err() {
        //     println!("{}", res.unwrap());
        //     println!("Got a request from {:?}", request.remote_addr());
        // } else{
        //     println!("{}", "Error!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        // }


        println!("Got a request from {:?}", request.remote_addr());

        let reply = funcloc::InvokeReply {
            result: format!("Hello {}!", request.into_inner().request),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn start_server(tctx: ThreadSafeContext<DetachedFromClient>) -> Result<(), Box<dyn std::error::Error>> {
    // thread::spawn(move || {
    //     for _ in 0..2 {
    //         let ctx = tctx.lock();
    //         ctx.call("INCR", &["threads"]).unwrap();
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // });


    let addr = "[::1]:50051".parse().unwrap();

    // let (tx, rx): (Sender<Invoke>, Receiver<Invoke>) = mpsc::channel();
    let executor = Arc::new(Executor::new(Mutex::new(tctx)));
    // let arc = Arc::new(executor);
    let exec = executor.clone();

    let greeter = MyGreeter::new(executor);
    // let (tx, rx) : (Sender<Invoke>, Receiver<Invoke>) = mpsc::channel();

    thread::spawn(move || {
        exec.run();
        // loop {
        //     round.run();
        //     // time::Duration::from_millis(1000);
        // }
    });


    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(FuncLocServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}