FROM rust:latest

# RUN apt update && apt install -y \
#  bash \
#  vim

WORKDIR /usr/rust-time-scheduler
CMD ["cargo", "run"]