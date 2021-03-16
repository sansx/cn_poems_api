cargo install systemfd cargo-watch
systemfd --no-pid -s http::PORT -- cargo watch -x run
