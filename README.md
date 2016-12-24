# fswatch
[![Travis](https://img.shields.io/travis/jkcclemens/fswatch.svg)](https://travis-ci.org/jkcclemens/fswatch)
[![Codecov](https://img.shields.io/codecov/c/github/jkcclemens/fswatch.svg)](https://codecov.io/gh/jkcclemens/fswatch)
[![Crates.io](https://img.shields.io/crates/v/fswatch.svg)](https://crates.io/crates/fswatch)
[![Docs.rs](https://img.shields.io/badge/docs-auto-blue.svg)](https://docs.rs/crate/fswatch)

This is a Rust crate to integrate with the C API of
[`libfswatch`](https://github.com/emcrisostomo/fswatch), via
[`fswatch-sys`](https://github.com/jkcclemens/fswatch-sys).

```rust
extern crate fswatch;

use fswatch::{Fsw, FswSession};

fn main() {
  // Initialize the library. This must be called before anything else can be done.
  Fsw::init_library().expect("Could not start fswatch");

  // Create a new session.
  let session = FswSession::default().unwrap();
  // Add a monitoring path, unwrapping any possible error.
  session.add_path("./").unwrap();
  // Set the callback for when events are fired, unwrapping any possible error.
  session.set_callback(|events| {
    // Prettily print out the vector of events.
    println!("{:#?}", events);
  }).unwrap();
  // Start the monitor, unwrapping any possible error. This will most likely be a blocking call.
  // See the libfswatch documentation for more information.
  session.start_monitor().unwrap();
}
```

```rust
extern crate fswatch;

use fswatch::{Fsw, FswSessionBuilder};

fn main() {
  Fsw::init_library().expect("Could not start fswatch");

  FswSessionBuilder::new(vec!["./"])
    .build_callback(|events| println!("{:#?}", events))
    .unwrap()
    .start_monitor()
    .unwrap();
}
```

```rust
extern crate fswatch;

use fswatch::{Fsw, FswSession};

fn main() {
  Fsw::init_library().expect("Could not start fswatch");

  let session = FswSession::builder().add_path("./").build().unwrap();
  for (_, event) in session {
    println!("{:#?}", event);
  }
}
```
