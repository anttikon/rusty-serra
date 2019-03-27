FROM rustlang/rust:nightly as builder

WORKDIR /app/src/rusty-serra

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /app
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

EXPOSE 8000

COPY --from=builder /app/src/rusty-serra/target/release/rusty-serra ./

CMD ["/app/rusty-serra"]
