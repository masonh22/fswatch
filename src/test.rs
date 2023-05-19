use std;
use std::sync::{Arc, Mutex, Condvar, Once};
use {ffi, Fsw, FswSession, FswError, FswMonitorType, FswMonitorFilter, FswFilterType, FswEvent};
use ffi::fsw_event_flag;

static FSW_INIT: Once = std::sync::Once::new();

fn initialize() {
  FSW_INIT.call_once(|| {
    Fsw::init_library().unwrap();
  });
}

fn get_default_session() -> FswSession {
  FswSession::default().unwrap()
}

fn create_sample_filter() -> FswMonitorFilter {
  FswMonitorFilter::new("\\w+\\.txt$", FswFilterType::Include, false, false)
}

#[test]
fn create_and_destroy_session() {
  initialize();
  {
    let _session = get_default_session();
  }
}

#[test]
fn create_session_from_builder() {
  initialize();
  FswSession::builder()
    .add_path("./")
    .property("test_name", "test_value")
    .overflow(Some(true))
    .monitor(FswMonitorType::SystemDefault)
    .latency(Some(1.0))
    .recursive(Some(true))
    .directory_only(Some(false))
    .follow_symlinks(Some(true))
    .add_event_filter(fsw_event_flag::Created)
    .add_filter(create_sample_filter())
    .build_callback(|events| println!("{:#?}", events))
    .unwrap();
}

#[test]
fn start_empty() {
  initialize();
  assert_eq!(Err(FswError::MissingRequiredParameters), get_default_session().start_monitor());
}

#[test]
fn start_without_callback() {
  initialize();
  let session = get_default_session();
  session.add_path("./").unwrap();
  assert_eq!(Err(FswError::MissingRequiredParameters), session.start_monitor());
}

#[test]
fn start_without_path() {
  initialize();
  let session = get_default_session();
  session.set_callback(|_| println!("Hello")).unwrap();
  assert_eq!(Err(FswError::MissingRequiredParameters), session.start_monitor());
}

#[test]
fn invalid_path() {
  initialize();
  let session = get_default_session();
  unsafe {
    assert!(ffi::fsw_add_path(session.handle(), ::std::ptr::null()) == ffi::FSW_ERR_INVALID_PATH);
  }
}

#[test]
fn run_callback() {
  initialize();
  // Get the cwd.
  let dir = std::env::current_dir().unwrap();
  // Define the file name for this test.
  let file_name = "fsw_test_file";

  // Create new condvar for waiting.
  let pair = Arc::new((Mutex::new(false), Condvar::new()));
  // Create clone for thread.
  let pair2 = pair.clone();
  // Create clone for thread.
  let dir2 = dir.clone();

  let (tx, rx) = std::sync::mpsc::channel();

  // Get a handle to this thread.
  let handle = std::thread::spawn(move || {
    // Extract our pair.
    let &(ref lock, ref cvar) = &*pair2;
    // Create a session.
    let session = FswSession::builder_paths(vec![dir2])
      .build().unwrap();
    // Send a signal to the main thread that we're ready for the file to be created.
    tx.send(()).unwrap();
    // Use the iterator pattern but immediately break out of it. This will leave the monitor running
    // and accumulating events if using below fswatch 1.10.0.
    for (s, event) in session {
      // Once we get an event, notify our waiting condvar and return the event.
      let mut started = lock.lock().unwrap();
      *started = true;
      cvar.notify_one();
      s.stop_monitor().unwrap();
      return event;
    }
    // This should be unreachable code, so panic if we get here.
    unreachable!();
  });

  // Generate the path for the file to create.
  let mut file_path = dir.clone();
  file_path.push(file_name);

  // Wait for the signal before creating the file.
  let _ = rx.recv().unwrap();

  // Wait one second for the loop to begin. // FIXME: there is a better way for this
  std::thread::sleep(std::time::Duration::from_secs(1));

  // Create the file for the thread to find the event for.
  std::fs::File::create(&file_path).unwrap();

  // Wait for the thread for up to five seconds.
  let &(ref lock, ref cvar) = &*pair;
  let mut started = lock.lock().unwrap();
  while !*started {
    let (s, timeout) = cvar.wait_timeout(started, std::time::Duration::from_secs(5)).unwrap();
    started = s;
    // Assert that we didn't time out waiting. This prevents the test from infinitely blocking.
    assert!(!timeout.timed_out());
  }

  // Get the event from the thread.
  let event: FswEvent = handle.join().unwrap();

  // Remove the file.
  std::fs::remove_file(&file_path).unwrap();

  let path = std::path::PathBuf::from(event.path);
  let event_file_name = path.file_name().unwrap().to_string_lossy();
  // Assert that the created file name matches the event's file name.
  assert_eq!(file_name, event_file_name);
}

#[test]
fn stop_monitor() {
  initialize();

  // Create a new, valid session.
  let session = FswSession::builder_paths(vec!["./"])
    .build_callback(|_| {})
    .unwrap();

  // Create an atomic reference count for the session.
  let arc = Arc::new(session);
  // Clone the Arc for the thread.
  let thread_session = arc.clone();
  // Create channel for sending signals.
  let (tx, rx) = std::sync::mpsc::channel();

  // Spawn the thread and get its handle for later.
  let handle = std::thread::spawn(move || {
    // Send the signal that the thread has started.
    tx.send(()).unwrap();
    // Start the monitor, which will block.
    thread_session.start_monitor().unwrap();
  });

  // Wait for the signal.
  rx.recv().unwrap();

  // Wait three seconds for the monitor to start. // FIXME: better way
  std::thread::sleep(std::time::Duration::from_secs(1));

  // Stop the monitor.
  arc.stop_monitor().unwrap();
  // Join from the thread, which should no longer be blocking.
  handle.join().unwrap();
}
