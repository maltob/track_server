FROM rust:latest as builder
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=track
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"
COPY ./Cargo.toml  .
COPY ./src/ /app/src/
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM debian:bullseye-slim
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
USER track:track
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/track_server ./
VOLUME /app/config/
CMD ["/app/track_server"]