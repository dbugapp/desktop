#!/bin/bash
EXE_NAME="dbug.exe"
TARGET="x86_64-pc-windows-msvc"
DBUG_VERSION=$(cat VERSION)

# update package version on Cargo.toml
cargo install cargo-edit
cargo set-version $DBUG_VERSION

# build binary
rustup target add $TARGET
cargo build --release --target=$TARGET
cp -fp target/$TARGET/release/$EXE_NAME target/release