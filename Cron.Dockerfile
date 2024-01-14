FROM ubuntu:22.04

# COPY target/release/cron to /app/cron
COPY ./target/release/cron /app/cron

CMD ["/app/cron"]
