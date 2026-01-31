FROM rust:alpine AS builder
WORKDIR /usr/src/sonar
COPY . .
RUN cargo build --release

FROM alpine AS runtime
RUN apk update && apk upgrade --no-cache
COPY --from=builder /usr/src/sonar/target/release/sonar /bin/sonar
ENTRYPOINT ["/bin/sonar"]
CMD ["--port", "3000", "--socket-path", "/var/run/docker.sock" ]