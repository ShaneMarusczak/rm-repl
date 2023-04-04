FROM rust:1.68.2-alpine AS builder

RUN apk add musl-dev

RUN rustup target add aarch64-unknown-linux-musl

WORKDIR /usr/src

RUN USER=root cargo new rmr

COPY Cargo.toml Cargo.lock /usr/src/rmr/

WORKDIR /usr/src/rmr

RUN cargo build --target aarch64-unknown-linux-musl --release

COPY src /usr/src/rmr/src/

RUN touch /usr/src/rmr/src/main.rs

RUN cargo build --target aarch64-unknown-linux-musl --release

FROM shinsenter/scratch as runtime

COPY --from=builder /usr/src/rmr/target/aarch64-unknown-linux-musl/release/rmr /

ENTRYPOINT ["/rmr"]
