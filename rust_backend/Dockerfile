FROM rust:1.80.1 AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN cargo fetch

COPY . .

RUN cargo build --release

CMD ["./target/release/rust_backend"]

FROM gcr.io/distroless/cc-debian12
# FROM gcr.io/distroless/base-debian12

COPY --from=builder /app/target/release/rust_backend .

EXPOSE 8081

CMD ["./rust_backend"]
