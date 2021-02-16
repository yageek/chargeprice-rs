#!/bin/sh

cd ../../
cargo build --lib -p chargeprice-jni
cp ./target/debug/libchargeprice_ffi.dylib ./bindings/libchargeprice_jni/macOS/libchargeprice_ffi.dylib
cp ./target/debug/libchargeprice_jni.dylib ./bindings/libchargeprice_jni/macOS/libchargeprice_jni.dylib
         