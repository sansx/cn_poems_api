cargo install systemfd cargo-watch  
## if your system is win7
cargo install cargo-watch --force --vers 4.0.3  
systemfd --no-pid -s http::PORT -- cargo watch -x run
