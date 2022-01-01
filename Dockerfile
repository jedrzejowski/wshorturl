FROM rust:alpine as backend

RUN apk add --no-cache musl-dev

COPY . /app
WORKDIR /app
RUN cargo install --path . --root /usr/local


FROM alpine

COPY --from=backend /usr/local/bin/wshorturl /usr/local/bin/wshorturl
COPY entrypoint.sh /entrypoint.sh

ENV WSHORTURL_HOST=0.0.0.0

ENTRYPOINT [ "/entrypoint.sh" ]
CMD [ "/usr/local/bin/wshorturl" ]
