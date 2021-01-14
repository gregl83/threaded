# threaded

Minimalist Thread Pool in Rust

## Stability

Experimental - Not for production use

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

# License

[MIT](LICENSE)
