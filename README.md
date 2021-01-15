[![Crates.io](https://img.shields.io/crates/v/threaded.svg)](https://crates.io/crates/threaded)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/threaded/blob/master/LICENSE)
![Build Status](https://github.com/gregl83/threaded/workflows/CI/badge.svg?branch=main)
# threaded

Minimalist Thread Pool in Rust

Glanceable source code for prototypes seeking brevity with transparency.

## Stability

Experimental

Threaded is not fully covered nor benched let alone comparatively performance tested.

## Branching Strategy

[Trunk Based Development](https://trunkbaseddevelopment.com/)

## Usage

```rust
use threaded::ThreadPool;

// start thread pool with fixed capacity of 2 workers (single producer, multiple consumer; spmc)
let tp = ThreadPool::new(2);

tp.execute(|| println!("hello threaded!")); // execute function in pool

let _num_workers = tp.capacity(); // get capacity of pool

// once tp goes out of scope, drop is called
// drop joins worker threads subsequently blocking main thread until workers finish

drop(tp); // manually trigger drop and join threads
```

See [docs.rs/threaded](https://docs.rs/threaded/0.2.0/threaded/).

## Credits

The [Rust Programming Language Book](https://doc.rust-lang.org/book/) details integral features needed to begin writing useful programs while adhering to community guidelines. Threaded was based directly off the book's final project ["Building a Multithreaded Web Server"](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).

# License

[MIT](LICENSE)
