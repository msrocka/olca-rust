cargo clean
cargo build --release

cp ./target/release/libolcar.so ./bin
