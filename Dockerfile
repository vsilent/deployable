FROM rust:slim as build

RUN apt-get update; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app;

WORKDIR /app

# copy manifests
COPY ./Cargo.toml .
COPY ./rustfmt.toml .
COPY ./Makefile .

# build this project to cache dependencies
RUN cargo build --release; \
    rm src/*.rs

# copy project source and necessary files
COPY ./src ./src

# add .env and secret.key for Docker env
RUN touch .env;

# rebuild app with project source
RUN rm -rf ./target/release/deps/deployable*; \
    cargo build --release

# deploy stage
FROM debian:bullseye-slim as production

# create app directory
WORKDIR /app

# copy binary and configuration files
COPY --from=build /app/target/release/deployable .
COPY --from=build /app/.env .

EXPOSE 8090

# run the binary
ENTRYPOINT ["/app/deployable"]