# Build openssl

See NOTES.ANDROID inside source directory of openssl

**NOTE**: A bug with ndk22 and openssl-1.1.j. See: https://github.com/openssl/openssl/pull/13694

patch -p1 < 12694.diff