# Prebuild dependencies
FROM rust:1.55 as build
RUN cargo new --bin blog
WORKDIR /blog
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/release/deps/blog*
RUN cargo build --release

# Final image
FROM rust:1.55-slim-buster
WORKDIR /blog
ENV ROCKET_ENV=release
COPY --from=build /blog/target/release/blog .
COPY Rocket.toml .
COPY ./frontend ./frontend
CMD ["./blog"]
