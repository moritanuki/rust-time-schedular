FROM rust:latest
WORKDIR /usr/src/projects

RUN cargo new time-schedular --bin
COPY . .
RUN cd /usr/src/projects/time-schedular && \
    cargo build

WORKDIR /usr/src/projects/time-schedular
CMD ["cargo", "run"]