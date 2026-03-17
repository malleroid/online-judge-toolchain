FROM rust:1.94.0-bookworm@sha256:ad263043ad351ba91388b6a4c7987cf1dbcab1163af0959348ecb4b3fef5faa3

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
