# Use the official Rust image as a parent image
# Use a base image with the latest version of Rust installed
FROM rust:latest AS builder

# Set the working directory in the Docker image
WORKDIR /usr/src/notification-api

# Copy the current directory contents into the container at /usr/src/notification-api
COPY ./src /usr/src/notification-api/src
COPY ./Cargo.toml /usr/src/notification-api
COPY ./Cargo.lock /usr/src/notification-api

# Compilation settings
ENV PKG_CONFIG_ALLOW_CROSS=1

# Rocket base configuration
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

# Compile the Rust application. Since we are building for release, use `cargo build --release`
RUN cargo build --release

FROM rust:latest AS runner

# Allow for unicode
ENV LANG=C.UTF-8

# Choose a workdir
WORKDIR /usr/local/bin

# Copy the statically-linked binary into this image. Make sure to use the correct path to your binary.
COPY --from=builder /usr/src/notification-api/target/release/notification-api .

# Run the application
CMD ["./notification-api"]