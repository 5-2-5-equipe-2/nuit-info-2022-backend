FROM rust:latest
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .
RUN cargo build --release
CMD ["cargo", "run", "--release"]