FROM rust as build
ADD ./Cargo.lock ./Cargo.toml /src/
ADD ./src /src/src/
WORKDIR /src
RUN cargo build --release 

FROM debian

RUN apt-get update -y; \
    apt-get install -y --no-install-recommends ca-certificates; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*;
RUN mkdir -p /usr/local/bin

COPY --from=build /src/target/release/aggregate_noobest /usr/local/bin/aggregate_noobest
CMD ["/usr/local/bin/aggregate_noobest"]