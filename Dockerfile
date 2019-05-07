FROM rust as build

COPY ./ ./

RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/botflox_alltalks /build-out

FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=build /build-out/botflox_alltalks /

ENTRYPOINT /botflox_alltalks