extern crate redis;
extern crate redis_module;

// use bytes::Bytes;
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::VecDeque;
// use crate::cmd::invoke::InvokeResult;
// use crate::cmd::Invoke;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicUsize, Ordering};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;
use std::time::Duration;
use std::cell::Cell;
use redis_module::{ThreadSafeContext, DetachedFromClient};
use libloading::os::unix::{Library, Symbol};
use std::pin::Pin;
use std::ops::{Generator, GeneratorState};
use crate::{Invoke, Task, TaskState};


//
// /// This enum represents the different states a task can be in.
// #[repr(u8)]
// #[derive(Clone, Copy, PartialEq)]
// pub enum TaskState {
//     /// A task is in this state when it has just been created, but has not
//     /// had a chance to execute on the CPU yet.
//     INITIALIZED = 0x01,
//
//     /// A task is in this state when it is currently running on the CPU.
//     RUNNING = 0x02,
//
//     /// A task is in this state when it has got a chance to run on the CPU at
//     /// least once, but has yeilded to the scheduler, and is currently not
//     /// executing on the CPU.
//     YIELDED = 0x03,
//
//     /// A task is in this state when it has finished executing completely, and
//     /// it's results are ready.
//     COMPLETED = 0x04,
//
//     /// A task is in this state when it has been stopped without completion, after
//     /// setting this state, the pushback mechanism will run.
//     STOPPED = 0x5,
//
//     /// A task is in this state when it has been suspended due to IO. On the client side
//     /// the task can wait for the native operation responses.
//     WAITING = 0x6,
// }
//
// /// This enum represents the priority of a task in the system. A smaller value
// /// indicates a task with a higher priority.
// #[repr(u8)]
// #[derive(Clone, PartialEq)]
// pub enum TaskPriority {
//     /// The priority of a dispatch task. Highest in the system, because this
//     /// task is responsible for all network processing.
//     DISPATCH = 0x01,
//
//     /// The priority of a task corresponding to an RPC request.
//     REQUEST = 0x02,
// }
//
// pub struct Container {
//     // The current state of the task. Required to determine if the task
//     // has completed execution.
//     state: TaskState,
//
//     // The priority of the task. Required to determine when the task should
//     // be run next, if it has not completed already.
//     priority: TaskPriority,
//
//     // The total amount of time in cycles the task has run for. Required to
//     // determine when the task should be run next, and for accounting purposes.
//     time: u64,
//
//     // The total amount of time in cycles the task has spend inside the database.
//     // Required to determine the credit for each run of an extension.
//     db_time: u64,
//
//     // An execution context for the task that implements the DB trait. Required
//     // for the task to interact with the database.
//     db: Cell<Option<Rc<Context>>>,
//
//     // The actual generator/coroutine containing the extension's code to be
//     // executed inside the database.
//     gen: Option<Pin<Box<dyn Generator<Yield = u64, Return = u64>>>>,
// }


pub struct Policy {
    pub tctx: Mutex<ThreadSafeContext<DetachedFromClient>>,
}

impl Policy {
    pub fn new(tctx: Mutex<ThreadSafeContext<DetachedFromClient>>) -> Policy {
        Policy {
            tctx: tctx,
        }
    }

    pub fn get(&self, key: &str) {
        let ctx = self.tctx.lock().unwrap().lock();
        ctx.call("GET", &[key]).unwrap();
    }

    pub fn set(&self, key: &str, data: &str) {
        let ctx = self.tctx.lock().unwrap().lock();
        ctx.call("SET", &[key, data]).unwrap();
    }
}



use redis::Commands;

fn fetch_data() -> redis::RedisResult<isize> {
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // throw away the result, just make sure it does not fail
    let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.get("my_key")
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get() {
        let val = fetch_data();
        println!("{}", val.unwrap());
    }
}