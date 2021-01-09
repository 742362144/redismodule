use redis_module::{redis_module,redis_command};
use redis_module::{Context, RedisError, RedisResult, ThreadSafeContext};
use std::thread;
use log::{error, info};
use std::error::Error;
use std::path::Path;
use std::{fmt, fs};

use time::Duration;
// use super::server::start_server;


fn  module(_: &Context, _args: Vec<String>) -> RedisResult {
    thread::spawn(move || {
        let thread_ctx = ThreadSafeContext::new();
        // start_server();
        for _ in 0..2 {
            let ctx = thread_ctx.lock();
            ctx.call("INCR", &["threads"]).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        // simple_logger::init().unwrap();
        // info!("Starting server...");
        //
        // let ip = "127.0.0.1:8594";
        //
        // let listener = TcpListener::bind(ip).expect("Unable to create listener.");
        // info!("Server started on: {}{}", "http://", ip);
        //
        // for stream in listener.incoming() {
        //     match stream {
        //         Ok(stream) => match handle_connection(stream) {
        //             Ok(_) => (),
        //             Err(e) => error!("Error handling connection: {}", e),
        //         },
        //         Err(e) => error!("Connection failed: {}", e),
        //     }
        // }
    });

    Ok(().into())
}

//////////////////////////////////////////////////////

redis_module! {
    name: "service",
    version: 1,
    data_types: [],
    commands: [
        ["service", service, "", 0, 0, 0],
    ],
}