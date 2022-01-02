#![no_main]
use libfuzzer_sys::fuzz_target;

use mdparse::parse;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        parse(s).unwrap();
    }
});
