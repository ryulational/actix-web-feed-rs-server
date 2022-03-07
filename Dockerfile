FROM rust:1.58.1
WORKDIR /app
COPY . .
RUN cargo build --release
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./target/release/rss_server"]
