#!/bin/sh
export OPENSSL_SOURCE_DIR=${HOME}/Downloads/openssl-1.1.1j
export ANDROID_VERSION=26

architectures=(android-arm64 android-arm android-x86 android-x86_64)

cd ${OPENSSL_SOURCE_DIR}
export ANDROID_NDK_HOME=${HOME}/Library/Android/sdk/ndk/22.0.7026061
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/darwin-x86_64/bin:$ANDROID_NDK_HOME/toolchains/arm-linux-androideabi-4.9/prebuilt/darwin-x86_64/bin:$PATH

# Loop over elements
for t in ${architectures[@]}; do
    perl ${OPENSSL_SOURCE_DIR}/Configure $t -D__ANDROID_API__=${ANDROID_VERSION} no-shared --prefix=$(pwd)/$t
    make -j 4
    make install_sw -j 4
done
