FROM rust:alpine AS builder
COPY . /usr/src/sonar
WORKDIR /usr/src/sonar

RUN cargo build --release


FROM alpine AS runtime

COPY --from=builder /usr/src/sonar/target/release/sonar /bin/sonar

ENTRYPOINT ["sonar"]
CMD ["--port", "3000", "--socket-path", "/var/run/docker.sock" ]