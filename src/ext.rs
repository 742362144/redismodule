#![feature(in_band_lifetimes)]
#![feature(generators, generator_trait)]
#![feature(llvm_asm)]

use std::sync::mpsc::Sender;
use std::pin::Pin;
use std::ops::{Generator, GeneratorState};
use std::sync::{Mutex, Arc};
use crate::executor::Policy;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::thread;
use std::time::Duration;

pub fn init(exec: Arc<Mutex<Policy>>) -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
// pub fn init() -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
//     print_hello();
    println!("{}", "enter");
    // let ctx = tctx.clone();
    // let tx = ctx.lock();

    println!("{}", "gen init");

    Box::pin(move || {
        let i:u64 = 1;
        exec.lock().unwrap().set("A", "111");
        println!("{}", "compute step 1 running");
        thread::sleep(Duration::from_millis(1000));
        // tx.call("SET", &["A", "1"]).unwrap();
        yield i;
        // tx.call("SET", &["B", "2"]).unwrap();
        println!("{}", "compute step 2 running");
        thread::sleep(Duration::from_millis(1000));
        yield i;
        // tx.call("SET", &["B", "2"]).unwrap();
        println!("{}", "compute step 3 running");
        thread::sleep(Duration::from_millis(1000));
        yield i;
        // tx.call("SET", &["B", "2"]).unwrap();
        println!("{}", "compute step 4 running");
        thread::sleep(Duration::from_millis(1000));
        yield i;
        // tx.call("SET", &["B", "2"]).unwrap();
        println!("{}", "compute step 5 running");
        thread::sleep(Duration::from_millis(1000));
        1111
    })
}