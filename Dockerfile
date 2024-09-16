FROM rust:1.81.0

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/mod_revenue_tracker"]
