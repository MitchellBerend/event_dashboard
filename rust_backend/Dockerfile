FROM rust:1.80.1 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder /app/target/release/rust_axum .

EXPOSE 8081

CMD ["./rust_axum"]
