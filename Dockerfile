# Use the official Rust image as a parent image
FROM rust:latest as builder

RUN apt install git

# Set the working directory in the Docker image
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# install trunk
RUN cargo install cargo-binstall
RUN cargo binstall trunk --no-confirm

RUN rustup target add wasm32-unknown-unknown

#EXPOSE 8000

# CMD [ "trunk", "serve" ]

# Build the project
RUN trunk build --release

FROM nginx:latest as server
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

# Use Debian as a base image for the final stage
# FROM debian:buster-slim

# Copy the binary from the builder stage to the final stage
# COPY --from=builder /usr/src/app/target/release/front-end /usr/local/bin


# Set the command to run your application
EXPOSE 8080

# Theres no need for a CMD or RUN because the NGINX image already has a CMD to start the server

#docker build -t fabdev-frontend .
#docker run -p 8000:8000 --name fabdev-front fabdev-frontend