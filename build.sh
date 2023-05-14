#!/bin/sh
echo '[.] Adding x86_64-pc-windows-gnu target ..'
rustup target add x86_64-pc-windows-gnu
echo '[.] Building with cargo..'
cargo build --target x86_64-pc-windows-gnu --release
echo '[+] Done- listing binary:'
ls target/x86_64-pc-windows-gnu/release/*.exe -l
