# nats-vs-http-bench
Simple benchmark NATS vs HTTP request-reply pattern

# How to run
* Build it: `cargo build --release`
* Run NATS server: `docker run -p 4222:4222 -p 8222:8222 -p 6222:6222 nats`
* Run `target/release/actix` and `target/release/nats`
* Run `cargo bench`

