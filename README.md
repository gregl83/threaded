[![Crates.io](https://img.shields.io/crates/v/threaded.svg)](https://crates.io/crates/threaded)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/gregl83/threaded/blob/master/LICENSE)
[![Build Status](https://github.com/gregl83/threaded/workflows/CI/badge.svg)](https://github.com/gregl83/threaded/actions?query=workflow%3ACI+branch%3Amaster) 
# threaded

Minimalist Thread Pool in Rust

Glanceable source code for prototypes seeking brevity with transparency.

## Stability

Experimental

Threaded is not fully covered nor benched let alone comparatively performance tested.

## Branching Strategy

[Trunk Based Development](https://trunkbaseddevelopment.com/)

## Usage

### Create ThreadPool

```rust
use threaded::ThreadPool;

// start thread pool with fixed capacity of 2 workers
let tp = ThreadPool::new(2);

// do something useful w/pool...

// once tp goes out of scope, drop is called
// drop joins worker threads subsequently blocking main thread until workers finish
```

### Execute Function using ThreadPool

```rust
// single producer, multiple consumer (spmc) thread pool with single function/closure execution
tp.execute(|| println!("hello threaded!"));
```

### Get Capacity of ThreadPool

```rust
let num_workers = tp.capacity();
```

### Trigger Drop of ThreadPool

```rust
drop(tp);
```

## Credits

The [Rust Programming Language Book](https://doc.rust-lang.org/book/) details integral features needed to begin writing useful programs while adhering to community guidelines. Threaded was based directly off the book's final project ["Building a Multithreaded Web Server"](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).

# License

[MIT](LICENSE)
