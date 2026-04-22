FROM rust:1.95.0-bookworm@sha256:6bb82db0878825e157664188b319c875de4f1fff5d70f5917b3a3f1974b472e4

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
