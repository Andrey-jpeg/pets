FROM rust:1-buster as builder
WORKDIR /usr/src/pets-api
COPY . .
RUN  cargo install --path .
FROM debian:bullseye-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies openssl & rm -rf /var/lib/apt/lists/* 
COPY --from=builder /usr/local/cargo/bin/pets-api /usr/local/bin/pets-api
ENTRYPOINT [ "pets-api" ]
EXPOSE 8080