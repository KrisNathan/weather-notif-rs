FROM ubuntu:22.04

RUN apt-get -y update
RUN apt-get -y upgrade
RUN apt-get -y install ca-certificates

# COPY target/release/server to /app/server
COPY ./target/release/server /app/server

CMD ["/app/server"]
