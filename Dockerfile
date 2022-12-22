FROM docker.io/library/rust:latest

# COPY target/release/weather-notif-rs /
COPY . /

RUN cargo build --release

CMD ["cargo", "run", "--release"]
