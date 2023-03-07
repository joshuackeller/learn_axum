FROM rust:latest as build

RUN USER=root cargo new --bin learn_axum
WORKDIR /learn_axum

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

RUN rm src/*.rs
COPY ./src ./src

RUN rm ./target/release/deps/learn_axum*
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build /learn_axum/target/release/learn_axum .

CMD ["./learn_axum"]