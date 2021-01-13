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
use crate::{Invoke, init, Task, TaskState};


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

    // pub fn run(&self) {
    //     // let exec = Arc::new(self);
    //     // let rec = self.data_rx.lock().unwrap();
    //
    //     loop {
    //         // let receive = self.data_rx.lock().unwrap().recv().unwrap();
    //         println!("{}", receive.cmd);
    //
    //     }
    // }

    pub fn get(&self, key: &str) {
        let ctx = self.tctx.lock().unwrap().lock();
        ctx.call("GET", &[key]).unwrap();
    }

    pub fn set(&self, key: &str, data: &str) {
        let ctx = self.tctx.lock().unwrap().lock();
        ctx.call("SET", &[key, data]).unwrap();
    }
}

pub struct Executor {
    pub policy: Arc<Mutex<Policy>>,
    pub waiting: RwLock<VecDeque<Box<Invoke>>>,
    pub running: RwLock<VecDeque<Box<Task>>>,
}

impl Executor {
    pub fn new(tctx: Mutex<ThreadSafeContext<DetachedFromClient>>) -> Executor {
        // let (tx, rx): (Sender<DataReq>, Receiver<DataReq>) = mpsc::channel();
        Executor {
            policy: Arc::new(Mutex::new(Policy::new(tctx))),
            waiting: RwLock::new(VecDeque::new()),
            running: RwLock::new(VecDeque::new()),
        }
    }

    pub fn add_task(&self, inv: Invoke) {
        self.waiting.write().unwrap().push_back(Box::new(inv));
    }

    pub fn run(&self) {
        loop {
            if self.waiting.read().unwrap().len() > 0 || self.running.write().unwrap().len() < 64 {
                let inv = self.waiting.write().unwrap().pop_front();
                if let Some(mut inv) = inv {
                    let pl = self.policy.clone();
                    let gen = init(pl);
                    self.running.write().unwrap().push_back(Box::new(Task::new(inv, Some(gen))));
                }

            }
            let task = self.waiting.write().unwrap().pop_front();

            let task = self.running.write().unwrap().pop_front();

            if let Some(mut task) = task {
                if task.run().0 == TaskState::COMPLETED {
                    println!("{}", "task finish.");
                    task.inv.tx.lock().unwrap().send(String::from("finish!"));
                } else {
                    println!("{}", "check server overhead.");
                    self.running.write().unwrap().push_back(task);
                }
            }
        }
    }
}

unsafe impl Send for Executor {}
unsafe impl Sync for Executor {}

// fn print_world(exec: Arc<&Executor>) {
//     exec.set("C", "3");
//     print!("{}", "dadad");
// }

// pub fn print_world() {
//     // let val = self.value.clone();
//     // Set the value in the shared database state.
//     // db.set(self.key, self.value, self.expire);
//
//     // type Proc = unsafe extern "C" fn(Rc<Db>) -> Pin<Box<Generator<Yield=u64, Return=InvokeResult>>>;
//     type Proc = unsafe extern "C" fn(Rc<&Db>) -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//     // type Proc = unsafe extern "C" fn(Rc<Db>) -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//     let library_path = String::from("/home/coder/IdeaProjects/storageloc/ext/add/target/debug/libadd.so");
//     println!("Loading add() from {}", library_path);
//
//     let lib = Library::new(library_path).unwrap();
//
//     unsafe {
//         let func: Symbol<Proc> = lib.get(b"init").unwrap();
//         let mut generator = func(Rc::new(db));
//
//         // println!("1");
//         // Pin::new(&mut generator).resume(());
//         // println!("3");
//         // let Some(GeneratorState<res1, res2>) = Pin::new(&mut generator).resume(());
//         // println!("5");
//
//         // db.set(String::from("c"), Bytes::from("dadada"), None);
//         match generator.as_mut().resume(()) {
//             GeneratorState::Yielded(1) => println!("Yielded"),
//             _ => panic!("unexpected return from resume"),
//         }
//         match generator.as_mut().resume(()) {
//             GeneratorState::Complete(1111) => println!("Completed"),
//             _ => panic!("unexpected return from resume"),
//         }
//         // println!("1 + 2 = {}", answer);
//     }
// }