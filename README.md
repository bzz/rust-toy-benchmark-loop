# Benchmark rust loop 

Simple benchmark of two way to loop in Rust.
Goal was to try Rust and check http://www.evanmiller.org/a-taste-of-rust.html example.


# os x pre-requests

```shell
brew install multirust
multirust update nightly
multirust default nightly
cargo new summ_bench --bin
touch src/lib.rs
```

# Build

```
cargo build
cargo bench
```


# Help

 1. http://doc.rust-lang.org/stable/book/benchmark-tests.html and [this comment](https://github.com/rust-lang/rust/issues/8191#issuecomment-26105181) helped.


 2. 2 build targets, 2 warnings (as without `#![feature(test)]` in main.rs code does not compile) and benchmark is run twice
  ```
src/lib.rs:1:1: 1:18 warning: unused attribute, #[warn(unused_attributes)] on by default
src/lib.rs:1 #![feature(test)]
             ^~~~~~~~~~~~~~~~~
src/lib.rs:1:1: 1:18 warning: crate-level attribute should be in the root module, #[warn(unused_attributes)] on by default
src/lib.rs:1 #![feature(test)]
             ^~~~~~~~~~~~~~~~~
  ```

 3. https://github.com/benaryorg/rust-loop_benchmarking and https://gist.github.com/llogiq/5790594ac45ce25015b3 did the same


 4. `time ./target/debug/summ_bench` results are not the same as in benchmarks

  ```
./target/debug/summ_bench  0.30s user 0.00s system 99% cpu 0.301 total
./target/debug/summ_bench  0.05s user 0.00s system 97% cpu 0.058 total

#VS

cargo bench
   Compiling summ_bench v0.1.0 (file:///.../rust/summ_bench)
     Running target/release/summ_bench-e8a8c9bf2c42419d

running 2 tests
test tests::bench_iter_summ  ... bench:   9,369,894 ns/iter (+/- 2,248,222)
test tests::bench_while_summ ... bench:   9,458,491 ns/iter (+/- 1,716,022)

test result: ok. 0 passed; 0 failed; 0 ignored; 2 measured
  ```