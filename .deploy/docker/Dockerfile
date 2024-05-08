FROM alpine:latest

COPY .deploy/docker/docker-entrypoint.sh /
COPY target/x86_64-unknown-linux-musl/release/web_server /usr/local/bin/web_server

WORKDIR /usr/local/bin
ENTRYPOINT ["sh", "/docker-entrypoint.sh"]
