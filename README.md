# rust-android-sample

## Setup

`~/.cargo/config`:
```
[target.x86_64-linux-android]
ar = "/opt/android_sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android-ar"
linker = "/opt/android_sdk/ndk-bundle/toolchains/llvm/prebuilt/darwin-x86_64/bin/x86_64-linux-android28-clang++"
```

```
$ rustup target add x86_64-linux-android
$ cargo build --target x86_64-linux-android --release
```

```
$ cp target/x86_64-linux-android/release/librust.so .../src/main/jniLibs/
```
