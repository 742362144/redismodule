// extern crate resp;
// extern crate rand;
//
// use std::io::BufReader;
// use resp::{Value, Decoder};
//
// pub fn get_str(len: usize) -> String {
//     use rand::Rng;
//     const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
//                             abcdefghijklmnopqrstuvwxyz\
//                             0123456789)(*&^%$#@!~";
//     let mut rng = rand::thread_rng();
//
//     let password: String = (0..len)
//         .map(|_| {
//             let idx = rng.gen_range(0, CHARSET.len());
//             CHARSET[idx] as char
//         })
//         .collect();
//     password
//
// }
//
// pub fn prepare_values(size: usize) -> Value {
//     let a = vec![
//         Value::String(get_str(size))];
//
//     Value::Array(a)
// }
//
// // all data as a large byte array
//
// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::thread;
//     use std::time::Duration;
//
//     // #[test]
//     // fn test_init() {
//     //     assert!(cycles_per_second() > 1000000000);
//     //     assert!(cycles_per_second() < 5000000000);
//     // }
//
//     #[test]
//     fn proto() {
//         let value = prepare_values(16);
//
//         value.encode();
//         assert!(1 < 50000);
//     }
// }
//
// // struct Net {
// //
// // }
//
