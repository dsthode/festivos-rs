FROM rust:1.64.0-alpine3.16

RUN apk add musl-dev
RUN rustup component add rustfmt
