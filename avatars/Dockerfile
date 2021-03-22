# Planner
FROM rust as planner
WORKDIR /avatars
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cacher
FROM ekidd/rust-musl-builder as cacher
RUN cargo install cargo-chef

WORKDIR /avatars

COPY --from=planner /avatars/recipe.json recipe.json

RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

# Builder
FROM ekidd/rust-musl-builder as builder
WORKDIR /avatars

COPY . .
COPY --from=cacher /avatars/target target

RUN cargo build --target x86_64-unknown-linux-musl --release

# Certs
FROM alpine:latest as certs
RUN apk --update add ca-certificates

# Release
FROM scratch
ENV PATH=/bin
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /avatars/target/x86_64-unknown-linux-musl/release/avatars /bin/avatars

ENTRYPOINT ["/bin/avatars"]