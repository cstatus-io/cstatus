// use curl::easy::{Easy2, Handler, WriteError};
// use futures::executor::block_on;
// use futures_timer::Delay;
// use std::thread;
// use std::time::Duration;

// struct Collector(Vec<u8>);

// impl Handler for Collector {
//     fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
//         self.0.extend_from_slice(data);
//         Ok(data.len())
//     }
// }

// async fn async_main() {
//     loop {
//         let mut easy = Easy2::new(Collector(Vec::new()));
//         easy.get(true).unwrap();
//         easy.url("https://caisy.io/").unwrap();
//         easy.perform().unwrap();
//         assert_eq!(easy.response_code().unwrap(), 200);
//         println!("response_code: {}", easy.response_code().unwrap());
//         println!("response_code: {:?}", easy.total_time().unwrap());
//         Delay::new(Duration::from_secs(1)).await;
//     }
// }

// fn main() {
//     env_logger::init();
//     let child = thread::spawn(move || {
//         block_on(async_main());
//     });
//     println!("lol1");

//     let res = child.join();
//     println!("lol2");
// }
