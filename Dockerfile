FROM haloboy777/rust-cache-layer:actix as build

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