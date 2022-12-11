FROM rust:latest
WORKDIR /usr/src/projects

COPY . .
RUN cargo build

CMD ["cargo", "run"]