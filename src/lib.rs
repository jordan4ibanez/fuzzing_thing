use std::{
  fs,
  io::{self, Seek, Write},
};

pub fn totally_safe_transmute<T, U>(v: T) -> U {
  #[repr(C)]
  enum E<T, U> {
    T(T),
    #[allow(dead_code)]
    U(U),
  }
  let v = E::T(v);

  let mut f = fs::OpenOptions::new()
    .write(true)
    .open("/proc/self/mem")
    .expect("welp");

  f.seek(io::SeekFrom::Start(&v as *const _ as u64))
    .expect("oof");
  f.write_all(&[1]).expect("darn");

  if let E::U(v) = v {
    return v;
  }

  panic!("rip");
}
