#!/bin/sh

cd ../../
cargo build --lib -p chargeprice-ffi
cp ./target/debug/libchargeprice_ffi.a ./bindings/libchargeprice_ffi/lib/macOS/libchargeprice_ffi.a
         