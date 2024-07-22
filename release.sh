#!/bin/sh
printf "Building for windows...\n"
cargo build --release --target x86_64-pc-windows-gnu

printf "\nBuilding for macos...\n"
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

printf "\nBuilding for linux...\n"
cargo build --release --target x86_64-unknown-linux-musl
cargo build --release --target aarch64-unknown-linux-musl

printf "\nCleaning up...\n"
rm -vrf ./release/
mkdir ./release/

printf "\nCollecting release builds...\n"
cp ./target/x86_64-pc-windows-gnu/release/deeztest.exe ./release/deeztest-windows-x64.exe

cp ./target/x86_64-apple-darwin/release/deeztest ./release/deeztest-macos-x64
cp ./target/aarch64-apple-darwin/release/deeztest ./release/deeztest-macos-arm64

cp ./target/x86_64-unknown-linux-musl/release/deeztest ./release/deeztest-linux-x64
cp ./target/aarch64-unknown-linux-musl/release/deeztest ./release/deeztest-linux-arm64

printf "\nShowing overview...\n"
ls -lah release/* 
file release/* 