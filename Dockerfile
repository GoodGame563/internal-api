FROM rust:1.84.1

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN apt-get install -y build-essential
RUN yes | apt install gcc-x86-64-linux-gnu
RUN yes | apt-get install protobuf-compiler

WORKDIR /app

COPY . /app
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'