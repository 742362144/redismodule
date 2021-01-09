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
use time;
use std::cell::Cell;


pub struct Invoke{
    pub tx: Sender<String>,  // send result
    pub req: String,
}

/// This enum represents the different states a task can be in.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum TaskState {
    /// A task is in this state when it has just been created, but has not
    /// had a chance to execute on the CPU yet.
    INITIALIZED = 0x01,

    /// A task is in this state when it is currently running on the CPU.
    RUNNING = 0x02,

    /// A task is in this state when it has got a chance to run on the CPU at
    /// least once, but has yeilded to the scheduler, and is currently not
    /// executing on the CPU.
    YIELDED = 0x03,

    /// A task is in this state when it has finished executing completely, and
    /// it's results are ready.
    COMPLETED = 0x04,

    /// A task is in this state when it has been stopped without completion, after
    /// setting this state, the pushback mechanism will run.
    STOPPED = 0x5,

    /// A task is in this state when it has been suspended due to IO. On the client side
    /// the task can wait for the native operation responses.
    WAITING = 0x6,
}

/// This enum represents the priority of a task in the system. A smaller value
/// indicates a task with a higher priority.
#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum TaskPriority {
    /// The priority of a dispatch task. Highest in the system, because this
    /// task is responsible for all network processing.
    DISPATCH = 0x01,

    /// The priority of a task corresponding to an RPC request.
    REQUEST = 0x02,
}

pub struct Container {
    // The current state of the task. Required to determine if the task
    // has completed execution.
    state: TaskState,

    // The priority of the task. Required to determine when the task should
    // be run next, if it has not completed already.
    priority: TaskPriority,

    // The total amount of time in cycles the task has run for. Required to
    // determine when the task should be run next, and for accounting purposes.
    time: u64,

    // The total amount of time in cycles the task has spend inside the database.
    // Required to determine the credit for each run of an extension.
    db_time: u64,

    // An execution context for the task that implements the DB trait. Required
    // for the task to interact with the database.
    db: Cell<Option<Rc<Context>>>,

    // The actual generator/coroutine containing the extension's code to be
    // executed inside the database.
    gen: Option<Pin<Box<dyn Generator<Yield = u64, Return = u64>>>>,
}


pub struct Executor {
    rx: Mutex<Receiver<Invoke>>,

    waiting: RwLock<VecDeque<Box<Task>>>,

}

impl Executor {
    pub fn new(rec: Mutex<Receiver<Invoke>> ) -> Executor {
        Executor {
            rx: rec,
            waiting: RwLock::new(VecDeque::new()),
        }
    }

    pub fn run(&self) {
        loop {
            let received = self.rx.lock().unwrap().recv().unwrap();
            println!("{}", received.req);
            let ten_millis = time::Duration::from_millis(1000);
            thread::sleep(ten_millis);

            received.tx.send(String::from("hello"));
        }
    }
}