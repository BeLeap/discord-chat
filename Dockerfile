FROM gcr.io/distroless/static-debian11

WORKDIR /app
COPY target/release/discord-bard-chat .

CMD ["./discord-bard-chat"]
