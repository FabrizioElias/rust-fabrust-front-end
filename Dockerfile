# Use the official Rust image as a parent image
FROM rust:latest as builder

# Set the working directory in the Docker image
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# install trunk
RUN cargo binstall trunk

# Build the project
RUN trunk build --release

# Use Debian as a base image for the final stage
FROM debian:buster-slim

# Copy the binary from the builder stage to the final stage
COPY --from=builder /usr/src/app/target/release/front-end /usr/local/bin

# Set the command to run your application
CMD ["front-end"]