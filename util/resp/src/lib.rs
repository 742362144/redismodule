#![feature(llvm_asm)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://avatars3.githubusercontent.com/u/15439811?v=3&s=200",
       html_favicon_url = "https://iorust.github.io/favicon.ico",
       html_root_url = "https://iorust.github.io",
       html_playground_url = "https://play.rust-lang.org",
       issue_tracker_base_url = "https://github.com/iorust/resp/issues")]

//! RESP(Redis Serialization Protocol) Serialization for Rust.

pub use self::value::Value;
pub use self::serialize::{encode, encode_slice, Decoder};

mod value;
mod serialize;

pub mod cycles;


pub fn get_str(len: usize) -> String {
       use rand::Rng;
       const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
       let mut rng = rand::thread_rng();

       let password: String = (0..len)
           .map(|_| {
                  let idx = rng.gen_range(0, CHARSET.len());
                  CHARSET[idx] as char
           })
           .collect();
       password

}

pub fn prepare_values(size: usize) -> Value {
       let a = vec![
              Value::String(get_str(size))];

       Value::Array(a)
}

// all data as a large byte array

#[cfg(test)]
mod test {
       use super::*;
       use std::thread;
       use std::time::Duration;


       // #[test]
       // fn proto() {
       //        let times = 10240;
       //        let mut i = 16;
       //        while i < 1024 * 1024 {
       //               let value = prepare_values(i);
       //               let start = cycles::rdtsc();
       //               let mut j = 0;
       //               while j< times {
       //                      let v = value.encode();
       //                      j = j + 1;
       //               }
       //
       //               let end = cycles::rdtsc();
       //               println!("{}", (end - start) / times);
       //               i = i * 2;
       //        }
       // }

       // #[test]
       // fn proto() {
       //        let times = 102400;
       //        let mut i = 102400;
       //        let value = prepare_values(i);
       //        let start = cycles::rdtsc();
       //        let mut j = 0;
       //        while j< times {
       //               let v = value.encode();
       //               j = j + 1;
       //        }
       //
       //        let end = cycles::rdtsc();
       //        println!("{}", (end - start) / times);
       // }
}
