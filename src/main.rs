use std::future::Future;
use std::pin::Pin;
use std::thread;

use crossbeam::channel;
use once_cell::sync::Lazy;

/// A queue that holds scheduled tasks.
static QUEUE: Lazy<channel::Sender<Task>> = Lazy::new(|| {
    // Create a queue.
    let (sender, receiver) = channel::unbounded::<Task>();

    // Spawn executor threads the first time the queue is created.
    for _ in 0..num_cpus::get().max(1) {
        let receiver = receiver.clone();
        thread::spawn(move || receiver.iter().for_each(|task| task.run()));
    }

    sender
});

/// A spawned future and its current state.
type Task = async_task::Task<()>;

/// Awaits the output of a spawned future.
type JoinHandle<R> = Pin<Box<dyn Future<Output = R> + Send>>;

/// Spawns a future on the executor.
fn spawn<F, R>(future: F) -> JoinHandle<R>
    where
        F: Future<Output = R> + Send + 'static,
        R: Send + 'static,
{
    // Create a task and schedule it for execution.
    let (task, handle) = async_task::spawn(future, |t| QUEUE.send(t).unwrap(), ());
    task.schedule();

    // Return a join handle that retrieves the output of the future.
    Box::pin(async { handle.await.unwrap() })
}

fn main() {
    futures::executor::block_on(async {
        // Spawn a future.
        let handle = spawn(async {
            println!("Running task...");
            1 + 2
        });

        // Await its output.
        assert_eq!(handle.await, 3);
    });
}