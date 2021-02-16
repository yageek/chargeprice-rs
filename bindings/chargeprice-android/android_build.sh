#!/bin/sh

# Set ANDROID PATH for building openssl
# export ANDROID_HOME=${HOME}/Library/Android/sdk/
# export ANDROID_NDK_HOME=${ANDROID_HOME}/ndk/21.1.6352462/
# export PATH=${ANDROID_NDK_HOME}/toolchains/llvm/prebuilt/darwin-x86_64/bin:${ANDROID_NDK_HOME}/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin/:${PATH}
export OPENSSL_BUILD_ROOT=~/Desktop/openssl_android
export OPENSSL_STATIC="1" # See: https://github.com/sfackler/rust-openssl/issues/383
export VERSION=""
cd ../../

# Arm 64
export OPENSSL_DIR=${OPENSSL_BUILD_ROOT}/arm64
cargo build --lib -p chargeprice-jni --target aarch64-linux-android ${VERSION}
cp ./target/aarch64-linux-android/debug/libchargeprice_jni.so ./bindings/libchargeprice_jni/android/jniLibs/arm64/libchargeprice_jni.so

# Armv7
export OPENSSL_DIR=${OPENSSL_BUILD_ROOT}/armeabi
cargo build --lib -p chargeprice-jni --target armv7-linux-androideabi ${VERSION}
cp ./target/armv7-linux-androideabi/debug/libchargeprice_jni.so ./bindings/libchargeprice_jni/android/jniLibs/armeabi/libchargeprice_jni.so

# x86
export OPENSSL_DIR=${OPENSSL_BUILD_ROOT}/x86
cargo build --lib -p chargeprice-jni --target i686-linux-android ${VERSION}
cp ./target/i686-linux-android/debug/libchargeprice_jni.so ./bindings/libchargeprice_jni/android/jniLibs/x86/libchargeprice_jni.so

# x86_64
export OPENSSL_DIR=${OPENSSL_BUILD_ROOT}/x86_64
cargo build --lib -p chargeprice-jni --target x86_64-linux-android ${VERSION}
cp ./target/x86_64-linux-android/debug/libchargeprice_jni.so ./bindings/libchargeprice_jni/android/jniLibs/x86_64/libchargeprice_jni.so
