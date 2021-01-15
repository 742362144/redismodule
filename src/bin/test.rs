// #![feature(generators, generator_trait)]
//
// // extern crate redis_module;
//
// // use redis_module::{ThreadSafeContext, DetachedFromClient};
// use std::sync::Arc;
// use std::pin::Pin;
// use std::ops::{Generator, GeneratorState};
// use libloading::os::unix::{Library, Symbol};
//
//
// fn main() {
//     // println!("{}", "test");
//     // let mut ctx = ThreadSafeContext::new();
//     // let b = Arc::new(&ctx);
//     // // type Proc = unsafe extern "C" fn(Rc<Db>) -> Pin<Box<Generator<Yield=u64, Return=InvokeResult>>>;
//     // type Proc = unsafe extern "C" fn(Arc<&ThreadSafeContext<DetachedFromClient>>) -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//
//     println!("{}", "f");
//
//
//     type Proc = unsafe extern "C" fn() -> Pin<Box<Generator<Yield=u64, Return=u64>>>;
//     let library_path = String::from("/home/coder/IdeaProjects/add/target/debug/libadd.so");
//     println!("Loading add() from {}", library_path);
//
//     let lib = Library::new(library_path).unwrap();
//
//     println!("{}", "1");
//     unsafe {
//         println!("{}", "2");
//         let func: Symbol<Proc> = lib.get(b"init").unwrap();
//         println!("{}", "3");
//         let mut generator = func();
//         println!("{}", "4");
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
//     }
// }
fn main() {


}