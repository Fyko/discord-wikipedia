ARG TARGET=x86_64-unknown-linux-musl
ARG BINARY_NAME=rust-template

FROM --platform=$TARGETPLATFORM clux/muslrust:1.73.0 AS chef
ARG TARGET
ARG BINARY_NAME
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target "$TARGET" --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY . .
RUN cargo build --release --target "$TARGET" --bin $BINARY_NAME

FROM --platform=$TARGETPLATFORM alpine:3.18.4 as runtime
ARG TARGET
ARG BINARY_NAME
RUN apk --no-cache add ca-certificates

COPY --from=builder /app/target/$TARGET/release/$BINARY_NAME /app/bin

CMD /app/bin
