# Example project that use Rust for AWS Lambda function

This example is tested within Mac environment.

## Project prerequisites

**Add target x86_64-unknown-linux-musl**

```
rustup target add x86_64-unknown-linux-musl
```

**Install a linker for the target platform**

```
brew install filosottile/musl-cross/musl-cross
```

## Build zip for AWS Lambda function

```
./build.sh
```
