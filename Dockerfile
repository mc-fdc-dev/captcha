FROM rust AS builder

WORKDIR /src/builder

RUN apt-get update && apt-get install -y musl-tools
RUN rustup target add x86_64-unknown-linux-musl

COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release -p discord

FROM alpine

WORKDIR /src/app

COPY --from=builder /src/builder/target/x86_64-unknown-linux-musl/release/discord .

CMD ["./discord"]