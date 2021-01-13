#![forbid(unsafe_code)]
#![feature(generators)]
#![feature(generator_trait)]

extern crate redismodule;

use std::ops::Generator;
use std::pin::Pin;

// use bytes::{Bytes, BytesMut, BufMut, Buf};

// use redis_module::{ThreadSafeContext, DetachedFromClient};
use redismodule::executor::Executor;
use std::sync::Arc;
// use log::{info, trace, warn};


#[no_mangle]
#[allow(unreachable_code)]
#[allow(unused_assignments)]
pub fn init(exec: Arc<Executor>) -> Pin<Box<Generator<Yield=u64, Return=u64> + '_>> {
// pub fn init() -> Pin<Box<Generator<Yield=u64, Return=u64> + 'static>> {
    println!("{}", "enter");
    // let ctx = tctx.clone();
    // let tx = ctx.lock();

    println!("{}", "1111");

    Box::pin(move || {
        let i:u64 = 1;
        println!("{}", "2222");
        // tx.call("SET", &["A", "1"]).unwrap();
        yield i;
        // tx.call("SET", &["B", "2"]).unwrap();
        println!("{}", "3333");
        1111
    })
}
// pub fn init(db: Rc<Db>) -> Pin<Box<Generator<Yield=u64, Return=&'static str>>> {
//     Box::pin(move || {
//         yield 1;
//         // yield 1;
//         println!("{}", "success");
//         let c = String::from("c");
//         let b = Bytes::from("bbbbbb");
//         db.set(c, b, None);
//         "foo"
//     })
// }



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
