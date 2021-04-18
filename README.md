# rust-logging-samples


An overview of rust's logging libraries. Provide a minimalistic set up for different logging libraries with the first four lines of `Mary has a little lamb`. See [Logging in Rust](https://medium.com/nerd-for-tech/logging-in-rust-e529c241f92e?source=friends_link&sk=a192567f131d4684c8e03da2891ba4cc) for more background information.

## Logging samples by libraries

### env_logger

`RUST_LOG=trace cargo run --bin env_logger`

```
[2021-04-11T23:49:20Z DEBUG env_logger] Mary has a little lamb
[2021-04-11T23:49:20Z ERROR env_logger] Its fleece was white as snow
[2021-04-11T23:49:20Z INFO  env_logger] "And every where that Mary went"
[2021-04-11T23:49:20Z WARN  env_logger] "The lamb was sure to go"
```

### log4rs

`cargo run --bin log4rs`

```
2021-04-11T17:04:30.024072700-07:00 DEBUG log4rs - Mary has a little lamb
2021-04-11T17:04:30.024185100-07:00 ERROR log4rs - Its fleece was white as snow
2021-04-11T17:04:30.024233300-07:00 INFO log4rs - "And every where that Mary went"
2021-04-11T17:04:30.024297300-07:00 WARN log4rs - "The lamb was sure to go"
```
