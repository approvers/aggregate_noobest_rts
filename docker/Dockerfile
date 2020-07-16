FROM rust:alpine as build
ADD ./src ./Cargo.lock ./Catgo.coml /src/
WORKDIR /src
RUN cargo build --release 

FROM alpine
COPY --from=build /src/target/release/aggregate_noobest /root
CMD ["/run"]