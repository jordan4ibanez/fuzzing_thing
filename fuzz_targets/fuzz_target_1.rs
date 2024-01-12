#![no_main]

use fuzzing_thing::totally_safe_transmute;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
  let test: String = totally_safe_transmute(data);
});
