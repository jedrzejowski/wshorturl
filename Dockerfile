FROM rust:alpine as backend

RUN apk add --no-cache musl-dev

COPY . /app
WORKDIR /app
RUN cargo install --path .


FROM alpine

COPY --from=backend /usr/local/bin/wshorturl /usr/local/bin/wshorturl
