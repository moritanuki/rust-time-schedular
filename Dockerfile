FROM rust:latest

RUN apt update && apt install -y \
 bash \
 vim

WORKDIR /usr/projects/src
CMD ["cargo", "run"]