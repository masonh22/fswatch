//! The FFI bindings.
//!
//! See the [C API docs](http://emcrisostomo.github.io/fswatch/doc/1.9.3/libfswatch.html/libfswatch_8h.html).

use {FswEventFlag, FswFilterType, FswMonitorType};
use libc::{c_int, c_uint, c_void, c_double, c_char, time_t};

#[link(name = "fswatch")]
extern "C" {
  pub fn fsw_init_library() -> FSW_STATUS;

  pub fn fsw_init_session(monitor_type: FswMonitorType) -> FSW_HANDLE;

  pub fn fsw_add_path(handle: FSW_HANDLE, path: *const c_char) -> FSW_STATUS;

  pub fn fsw_add_property(handle: FSW_HANDLE, name: *const c_char, value: *const c_char) -> FSW_STATUS;

  pub fn fsw_set_allow_overflow(handle: FSW_HANDLE, allow_overflow: bool) -> FSW_STATUS;

  pub fn fsw_set_callback(handle: FSW_HANDLE, callback: FSW_CEVENT_CALLBACK, data: *const c_void) -> FSW_STATUS;

  pub fn fsw_set_latency(handle: FSW_HANDLE, latency: c_double) -> FSW_STATUS;

  pub fn fsw_set_recursive(handle: FSW_HANDLE, recursive: bool) -> FSW_STATUS;

  pub fn fsw_set_directory_only(handle: FSW_HANDLE, directory_only: bool) -> FSW_STATUS;

  pub fn fsw_set_follow_symlinks(handle: FSW_HANDLE, follow_symlinks: bool) -> FSW_STATUS;

  pub fn fsw_add_event_type_filter(handle: FSW_HANDLE, event_type: fsw_event_type_filter) -> FSW_STATUS;

  pub fn fsw_add_filter(handle: FSW_HANDLE, filter: fsw_cmonitor_filter) -> FSW_STATUS;

  pub fn fsw_start_monitor(handle: FSW_HANDLE) -> FSW_STATUS;

  #[cfg(feature = "1_10_0")]
  pub fn fsw_stop_monitor(handle: FSW_HANDLE) -> FSW_STATUS;

  pub fn fsw_destroy_session(handle: FSW_HANDLE) -> FSW_STATUS;

  pub fn fsw_last_error() -> FSW_STATUS;

  pub fn fsw_is_verbose() -> bool;

  pub fn fsw_set_verbose(verbose: bool);
}

pub type FSW_STATUS = c_int;
cfg_if! {
  if #[cfg(feature = "1_10_0")] {
    #[derive(Debug, PartialEq)]
    pub enum FSW_SESSION {}

    pub type FSW_HANDLE = *mut FSW_SESSION;
    pub const FSW_INVALID_HANDLE: FSW_HANDLE = ::std::ptr::null_mut();
  } else {
    pub type FSW_HANDLE = c_int;
    pub const FSW_INVALID_HANDLE: FSW_HANDLE = -1;
  }
}
pub type FSW_CEVENT_CALLBACK = extern fn(events: *const fsw_cevent, event_num: c_uint, data: *mut c_void);

pub const FSW_OK: FSW_STATUS = 0;
pub const FSW_ERR_UNKNOWN_ERROR: FSW_STATUS = 1;
pub const FSW_ERR_SESSION_UNKNOWN: FSW_STATUS = (1 << 1);
pub const FSW_ERR_MONITOR_ALREADY_EXISTS: FSW_STATUS = (1 << 2);
pub const FSW_ERR_MEMORY: FSW_STATUS = (1 << 3);
pub const FSW_ERR_UNKNOWN_MONITOR_TYPE: FSW_STATUS = (1 << 4);
pub const FSW_ERR_CALLBACK_NOT_SET: FSW_STATUS = (1 << 5);
pub const FSW_ERR_PATHS_NOT_SET: FSW_STATUS = (1 << 6);
pub const FSW_ERR_MISSING_CONTEXT: FSW_STATUS = (1 << 7);
pub const FSW_ERR_INVALID_PATH: FSW_STATUS = (1 << 8);
pub const FSW_ERR_INVALID_CALLBACK: FSW_STATUS = (1 << 9);
pub const FSW_ERR_INVALID_LATENCY: FSW_STATUS = (1 << 10);
pub const FSW_ERR_INVALID_REGEX: FSW_STATUS = (1 << 11);
pub const FSW_ERR_MONITOR_ALREADY_RUNNING: FSW_STATUS = (1 << 12);
pub const FSW_ERR_UNKNOWN_VALUE: FSW_STATUS = (1 << 13);
pub const FSW_ERR_INVALID_PROPERTY: FSW_STATUS = (1 << 14);

#[derive(Debug)]
#[repr(C)]
pub struct fsw_event_type_filter {
  pub flag: FswEventFlag
}

#[derive(Debug)]
#[repr(C)]
pub struct fsw_cmonitor_filter {
  pub text: *const c_char,
  pub filter_type: FswFilterType,
  pub case_sensitive: bool,
  pub extended: bool
}

#[derive(Debug)]
#[repr(C)]
pub struct fsw_cevent {
  pub path: *const c_char,
  pub evt_time: time_t,
  pub flags: *const FswEventFlag,
  pub flags_num: c_uint
}
