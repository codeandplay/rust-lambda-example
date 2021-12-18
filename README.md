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

## Lambda function logging level

Logging for this project is using [simple_logger](https://docs.rs/simple_logger/1.16.0/simple_logger/)
When the lambda function is running within AWS, The logging level can be changed by setting the environment variable for **RUST_LOG**, e.g **RUST_LOG=info**
