#![feature(in_band_lifetimes)]
#![feature(generators, generator_trait)]

#[macro_use]
extern crate redis_module;

use std::sync::mpsc::Sender;
use std::pin::Pin;
use std::ops::Generator;
use std::sync::{Mutex, Arc};
use crate::executor::Policy;

pub mod server;

pub mod executor;
pub mod module;

pub struct Invoke{
    pub tx: Sender<String>,  // send result
    pub req: String,
}

pub struct DataReq{
    pub tx: Sender<String>,  // send result
    pub cmd: String,
}

pub fn init(exec: Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
// pub fn init() -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
//     print_hello();
    println!("{}", "enter");
    // let ctx = tctx.clone();
    // let tx = ctx.lock();

    println!("{}", "1111");

    Box::pin(move || {
        let i:u64 = 1;
        exec.lock().unwrap().set("A", "111");
        println!("{}", "2222");
        // tx.call("SET", &["A", "1"]).unwrap();
        yield i;
        // tx.call("SET", &["B", "2"]).unwrap();
        println!("{}", "3333");
        1111
    })
}