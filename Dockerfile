FROM rust:1.95.0-bookworm@sha256:225aa827d55fae9816a0492284592827e794a5247c6c6a961c3b471b344295ec

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
