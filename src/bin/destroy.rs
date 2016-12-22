extern crate fswatch;

use fswatch::ffi;

fn main() {
  unsafe {
    ffi::fsw_init_library();
    let session = ffi::fsw_init_session(ffi::fsw_monitor_type::system_default_monitor_type);
    ffi::fsw_destroy_session(session);
  }
}
