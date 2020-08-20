Followed steps here for basic Lambda setup: https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

## To deploy

    cargo build --release --target x86_64-unknown-linux-musl

    zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap graph.json