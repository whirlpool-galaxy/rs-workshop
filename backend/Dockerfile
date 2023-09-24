FROM rust:alpine as build

RUN apk update --no-cache
RUN apk upgrade --no-cache
RUN apk add --no-cache musl-dev
RUN apk add --no-cache openssl-dev

WORKDIR /app

RUN --network=host \
    --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/rs-workshop /bin/rs-workshop
EOF

FROM alpine

RUN apk update --no-cache
RUN apk upgrade --no-cache

COPY --from=build /bin/rs-workshop /bin/

CMD ["/bin/rs-workshop"]