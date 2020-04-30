FROM rust:1.42 as builder
WORKDIR /usr/src/myapp
COPY . /tmp/
RUN cargo install --path /tmp/ --root /tmp/
RUN ls /tmp/bin/

FROM ubuntu:19.10
COPY --from=builder /tmp/bin/telegram-github-acttion /telegram-github-acttion
RUN chmod +x /telegram-github-acttion
RUN apt-get update
RUN apt-get install openssl -y
RUN apt-get install ca-certificates
ENTRYPOINT /telegram-github-acttion