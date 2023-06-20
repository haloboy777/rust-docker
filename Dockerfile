# Base image extends debian:buster-slim
FROM rust:slim-bookworm as build

# Init Repo for building dependencies and caching them for later use
WORKDIR /tmp
RUN cargo init --vcs none main
COPY Cargo.toml ./main
WORKDIR /tmp/main
RUN cargo build --release

# Compile main
RUN rm -rf /tmp/main/src/
COPY src/ /tmp/main/src/
RUN touch src/main.rs

RUN cargo build --release

# Copy the binary into a new container for a smaller docker image
FROM debian:stable-slim
COPY --from=build /tmp/main/target/release/rust-docker /executor
USER root

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full
EXPOSE 8080

CMD ["/executor"]