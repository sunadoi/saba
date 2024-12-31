#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

fn main() -> u64 {
  let client = HttpClient::new();
  match client.get("example.com".to_string(), 80, "/".to_string()) {
    Ok(response) => {
      println!("Response: \n{:#?}", response);
    }
    Err(e) => {
      println!("Error: \n{:#?}", e);
    }
  }
  0
}

entry_point!(main);
