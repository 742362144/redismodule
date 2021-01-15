// #![feature(in_band_lifetimes)]
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

pub mod server;

pub mod executor;
pub mod module;

pub mod cycles;

pub mod model;
pub mod policy;
pub mod ext;


pub struct Invoke{
    pub tx: Mutex<Sender<String>>,  // send result
    pub req: String,
}


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

pub struct Task {
    state: TaskState,
    time: u64,
    inv: Box<Invoke>,
    gen: Option<Pin<Box<dyn Generator<Yield = u64, Return = u64>>>>,
}

impl Task {
    pub fn new(inv: Box<Invoke>, gen: Option<Pin<Box<dyn Generator<Yield = u64, Return = u64>>>>) -> Task {
        Task {
            state: TaskState::INITIALIZED,
            time: 0,
            inv,
            gen,
        }
    }

    fn run(&mut self) -> (TaskState, u64) {
        let start = cycles::rdtsc();

        // Resume the task if need be. The task needs to be run/resumed only
        // if it is in the INITIALIZED or YIELDED state. Nothing needs to be
        // done if it has already completed, or was aborted.
        if self.state == TaskState::INITIALIZED || self.state == TaskState::YIELDED {
            self.state = TaskState::RUNNING;

            // Catch any panics thrown from within the extension.
            let res = catch_unwind(AssertUnwindSafe(|| match self.gen.as_mut() {
                Some(gen) => match gen.as_mut().resume(()) {
                    GeneratorState::Yielded(_) => {
                        println!("{}", "yield...");
                        self.state = TaskState::YIELDED;
                    }

                    GeneratorState::Complete(_) => {
                        self.state = TaskState::COMPLETED;
                        println!("{}", "complete...");
                    }
                },

                None => {
                    panic!("No generator available for extension execution");
                }
            }));

            // If there was a panic thrown, then mark the container as COMPLETED so that it
            // does not get run again.
            // if let Err(_) = res {
            //     self.state = TaskState::COMPLETED;
            //     if thread::panicking() {
            //         // Wait for 100 millisecond so that the thread is moved to the GHETTO core.
            //         let start = cycles::rdtsc();
            //         while cycles::rdtsc() - start < cycles::cycles_per_second() / 10 {}
            //     }
            // }
        }

        // Calculate the amount of time the task executed for in cycles.
        let exec = cycles::rdtsc() - start;

        // Update the total execution time of the task.
        self.time += exec;

        // Return the state and the amount of time the task executed for.
        return (self.state, exec);
    }
}