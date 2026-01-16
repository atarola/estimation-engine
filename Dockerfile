# syntax = docker/dockerfile:1

#
# Yarn Build
#

FROM node:24 AS yarn_builder

WORKDIR /app
COPY . .

RUN npm install -g yarn@1.22.22 --force
RUN yarn install --frozen-lockfile && \
    yarn clean && \
    yarn build 

#
# Rust Build
# 

FROM rust:1.86 AS rust_builder

WORKDIR /app
COPY . .
COPY --from=yarn_builder /app/public ./public

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

#
# Final Container
#

FROM alpine:latest 
RUN apk update && apk add ca-certificates && rm -rf /var/cache/apk/*
WORKDIR /usr/local/bin
COPY --from=rust_builder /app/target/x86_64-unknown-linux-musl/release/estimation-engine .
EXPOSE 3000
CMD ["estimation-engine"]

