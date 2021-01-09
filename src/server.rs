use std::sync::{mpsc, Mutex, Arc};
use std::sync::mpsc::{Sender, Receiver};

use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use std::thread;

use super::executor::{Executor, Invoke};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

// #[derive(Default)]
pub struct MyGreeter {
    tx: Mutex<Sender<Invoke>>,
}

impl MyGreeter {
    pub fn new(tx: Sender<Invoke>) -> MyGreeter {
        MyGreeter{
            tx: Mutex::new(tx),
        }
    }
}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {

        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

        self.tx.lock().unwrap().send(Invoke{tx, req: String::from("hello")});

        let res = rx.recv().unwrap();
        println!("{}", res);
        println!("Got a request from {:?}", request.remote_addr());

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

#[tokio::main]
pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();

    let (tx, rx): (Sender<Invoke>, Receiver<Invoke>) = mpsc::channel();
    let executor = Executor::new(Mutex::new(rx));
    // let arc = Arc::new(executor);
    let greeter = MyGreeter::new(tx);
    // let (tx, rx) : (Sender<Invoke>, Receiver<Invoke>) = mpsc::channel();


    thread::spawn(move || {
        executor.run();
        // loop {
        //     round.run();
        //     // time::Duration::from_millis(1000);
        // }
    });



    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}