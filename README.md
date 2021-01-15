# threaded

Minimalist Thread Pool in Rust

Glanceable source code for prototypes seeking brevity with transparency.

## Stability

Experimental

Threaded is not fully covered nor benched let along comparatively performance tested.

## Branching Strategy

[Trunk Based Development](https://trunkbaseddevelopment.com/)

## Usage

```rust
use threaded::ThreadPool;

// start thread pool with size of 2 workers
let tp = ThreadPool::new(2);

// execute job/closure using worker in thread pool
tp.execute(|| println!("hello threaded!"));

// tp out of scope triggering drop call
// worker threads are joined resulting in block on main thread until workers finish
```

## Credits

The [Rust programming language book](https://doc.rust-lang.org/book/) details key integral features needed to begin writing useful programs while adhering to community guidelines. Threaded was based directly off the book's final project ["A Web Server"](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html).

# License

[MIT](LICENSE)
