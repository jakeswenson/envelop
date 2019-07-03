use libc::c_int;
use envelop;

#[no_mangle]
pub extern "C" fn swig(_a: c_int, _b: c_int) -> c_int {
  return 0;
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
