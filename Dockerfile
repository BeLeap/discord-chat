FROM rust AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM rust:slim

WORKDIR /app
COPY --from=builder /app/target/release/discord-bard-chat .

CMD ["./discord-bard-chat"]

