# Use the official Rust image.
# https://hub.docker.com/_/rust
FROM rust as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin soreike
WORKDIR /soreike

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4 Now that the dependency is built, coy your source code
COPY ./src ./src

# 5. Build for release.
RUN rm ./target/release/deps/soreike*
RUN cargo install --path .

# our final base
FROM rust:slim

# copy the build artifact from the build stage
COPY --from=build /soreike/target/release/soreike .

# Service must listen to $PORT environment variable.
# This default value facilitates local development.
ENV PORT 8080

# Run the web service on container startup.
ENTRYPOINT ["./soreike"]