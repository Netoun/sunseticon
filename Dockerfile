FROM node:alpine as builder-node

WORKDIR /usr/src/app

COPY ./vue .

RUN \
  yarn; \
yarn build;

FROM debian:stretch-slim as builder-rust

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        gcc \
        libc6-dev \
        wget \
        ; \
    \
    url="https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --default-toolchain nightly; \
    rm rustup-init; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup --version; \
    cargo --version; \
    rustc --version; \
    \
    apt-get remove -y --auto-remove \
        wget \
        ; \
    rm -rf /var/lib/apt/lists/*;

WORKDIR /usr/src/app

COPY ./back .

ENV PKG_CONFIG_ALLOW_CROSS=1

RUN cargo build  --release

FROM rust:slim

COPY --from=builder-rust /usr/src/app/target/release/sunseticon .
COPY --from=builder-node /usr/src/app/dist .

CMD ["./sunseticon"]