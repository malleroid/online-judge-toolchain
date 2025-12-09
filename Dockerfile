FROM rust:1.91.1-bookworm@sha256:5b43be93ef3a360c4f84dfabc962724ea4c164031c641a182897ebf69c59987d

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
