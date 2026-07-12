FROM rust:1.96.1-bookworm AS build

WORKDIR /app

COPY Cargo.lock Cargo.toml ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY ./src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /usr/local/bin
COPY --from=build /app/target/release/Resume_TUI .
COPY resume.toml .
CMD ["./Resume_TUI"]