FROM rust:1.93.1-bookworm@sha256:c38b1b917cb749e50aea7dd6e87f6e315d62a4bc84e38d63f5eb8b1908db1b9a

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
