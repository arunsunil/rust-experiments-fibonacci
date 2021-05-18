FROM alpine:latest

WORKDIR /root
COPY ./target/release/rust-experiments-fibonacci ./rust-experiments-fibonacci

ENTRYPOINT ["./rust-experiments-fibonacci"]
