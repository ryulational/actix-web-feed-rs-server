FROM rust:1.57.0
WORKDIR /app
COPY . .
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/rss_server"]
