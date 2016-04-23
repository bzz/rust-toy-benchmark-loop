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

  cargo build
  cargo bench



# Frustrating

 1. http://doc.rust-lang.org/stable/book/benchmark-tests.html not realy helpfull in case you have a binary and library. [This comment](https://github.com/rust-lang/rust/issues/8191#issuecomment-26105181) helped

  Still there are warnings, as without `#![feature(test)]` in main.rs code does not compile
  ```
src/lib.rs:1:1: 1:18 warning: unused attribute, #[warn(unused_attributes)] on by default
src/lib.rs:1 #![feature(test)]
             ^~~~~~~~~~~~~~~~~
src/lib.rs:1:1: 1:18 warning: crate-level attribute should be in the root module, #[warn(unused_attributes)] on by default
src/lib.rs:1 #![feature(test)]
             ^~~~~~~~~~~~~~~~~
  ```