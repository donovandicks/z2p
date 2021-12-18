# Chef Stage - base
# Initializes the base image
FROM lukemathwalker/cargo-chef:latest-rust-1-slim as chef
WORKDIR /app

# Planner Stage
# Generates a lock-like file for faster builds
FROM chef as planner

COPY . .

## Compute a lock-like file
RUN cargo chef prepare --recipe-path recipe.json

# Build Stage
# Compiles the project
FROM chef AS build

COPY --from=planner /app/recipe.json recipe.json

## Build dependencies but not application
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

## Enables successful compilation/check of sqlx queries in environments without
## an online database instance
ENV SQLX_OFFLINE true

RUN cargo build --release --bin z2p

# Runtime image
# Runs the actual binary
FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

## Copy the release binary from build
COPY --from=build /app/target/release/z2p z2p

COPY configuration configuration

ENV APP_ENVIRONMENT production

EXPOSE 8000

ENTRYPOINT ["./z2p"]
