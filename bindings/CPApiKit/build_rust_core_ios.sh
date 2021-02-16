#!/bin/sh

cd ../../
cargo build --target=x86_64-apple-ios
cargo +ios-arm64-nightly-2021-01-25 build --target aarch64-apple-ios
libtool -static -o bindings/libchargeprice_ffi/lib/iOS/libchargeprice_ffi.a \
                 ./target/aarch64-apple-ios/debug/libchargeprice_ffi.a \
                 ./target/x86_64-apple-ios/debug/libchargeprice_ffi.a            