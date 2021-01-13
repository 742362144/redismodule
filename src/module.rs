use redis_module::{redis_module, redis_command};
use redis_module::{Context, RedisError, RedisResult, ThreadSafeContext};
use std::thread;
use log::{error, info};
use std::error::Error;
use std::path::Path;
use std::{fmt, fs};

use std::time::Duration;
use crate::server::start_server;


fn  service(_: &Context, _args: Vec<String>) -> RedisResult {
    thread::spawn(move || {
        let thread_ctx = ThreadSafeContext::new();
        start_server(thread_ctx);
        // print_world();

        // for _ in 0..2 {
        //     let ctx = thread_ctx.lock();
        //     ctx.call("INCR", &["threads"]).unwrap();
        //     thread::sleep(Duration::from_millis(100));
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