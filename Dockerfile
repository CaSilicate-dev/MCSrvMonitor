FROM debian:stable
WORKDIR /app
COPY ./target/release/MCSrvMonitor /app


CMD ["./MCSrvMonitor"]