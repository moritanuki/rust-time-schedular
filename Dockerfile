FROM rust:latest

RUN cargo install cargo-watch
# RUN apt update && apt install -y \
#  bash \
#  vim

WORKDIR /usr/rust-time-scheduler