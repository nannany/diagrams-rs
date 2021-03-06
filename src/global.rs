use std::env;

use once_cell::sync::Lazy;

pub static LARGE_TEXT: Lazy<String> =
    Lazy::new(|| env::current_dir().unwrap().to_str().unwrap().to_string());

pub static mut LARGE_ID: Lazy<u32> = Lazy::new(|| 0);
