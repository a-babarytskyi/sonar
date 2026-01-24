FROM rust:latest as builder
COPY . /usr/src/sonard
WORKDIR /usr/src/sonard

RUN cargo build --release


FROM debian:bookworm-slim as runtime

COPY --from=builder /usr/src/sonard/target/release/sonard /bin/sonard

ENTRYPOINT ["sonard"]
CMD ["--port", "3000", "--socket-path", "/var/run/docker.sock" ]