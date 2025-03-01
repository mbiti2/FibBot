FROM rust:alpine AS build

WORKDIR /app

COPY . .

RUN apk add --no-cache openssl-dev musl-dev pkgconf 
RUN apk add --no-cache build-base clang lld


RUN cargo build --release

# RUN docker build --no-cache --progress=plain .


FROM alpine:latest

WORKDIR /app

RUN apk add libgcc

COPY --from=build /app/target/release/fib-bot /app/fib-bot

ENTRYPOINT [ "/app/fib-bot" ]
