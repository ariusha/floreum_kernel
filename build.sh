#!/bin/sh
TARGET="x86_64-unknown-none"
TARGET_NAME="amd64"
cargo build --target $TARGET --release
cp ./target/$TARGET/release/floreum_kernel ./drive/kernel/default.$TARGET_NAME
cp ./LICENSE ./drive/kernel/FLOREUM-LICENSE
cp ./README ./drive/kernel/FLOREUM-README