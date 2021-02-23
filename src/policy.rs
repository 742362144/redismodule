extern crate runtime;
extern crate redis_module;

use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;
use redis_module::{ThreadSafeContext, DetachedFromClient};
use std::pin::Pin;
use runtime::task::{Task, TaskState};
use runtime::invoke::{Invoke};

use runtime::policy::Policy;
use redis::RedisResult;
use self::redis_module::{RedisString, RedisValue};


pub struct LocalPolicy {
    pub tctx: Mutex<ThreadSafeContext<DetachedFromClient>>,
}

impl LocalPolicy {
    pub fn new(tctx: Mutex<ThreadSafeContext<DetachedFromClient>>) -> LocalPolicy {
        LocalPolicy {
            tctx,
        }
    }
}

impl Policy for LocalPolicy {
    fn get(&mut self, key: &str) -> String {
        let ctx = self.tctx.lock().unwrap().lock();
        let res = ctx.call("GET", &[key]);
        match res {
            Ok(RedisValue::SimpleString(v)) => {
                v
            }
            _ => String::from(""),
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        let ctx = self.tctx.lock().unwrap().lock();
        ctx.call("SET", &[key, value]).unwrap();
    }
}


// use redis::Commands;
//
// fn fetch_data() -> redis::RedisResult<isize> {
//     // connect to redis
//     let client = redis::Client::open("redis://127.0.0.1/")?;
//     let mut con = client.get_connection()?;
//     // throw away the result, just make sure it does not fail
//     let _ : () = con.set("my_key", 42)?;
//     // read back the key and return it.  Because the return value
//     // from the function is a result for integer this will automatically
//     // convert into one.
//     con.get("my_key")
// }
//
//
// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn get() {
//         let val = fetch_data();
//         println!("{}", val.unwrap());
//     }
// }