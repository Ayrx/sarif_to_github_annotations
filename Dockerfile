FROM rust:latest AS builder
WORKDIR /build
COPY ./ ./
RUN cargo build --release

FROM ubuntu:latest
COPY --from=builder /build/target/release/sarif_to_github_annotations /sarif_to_github_annotations
COPY entrypoint.sh /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
