FROM rust:1.94.0-bookworm@sha256:ca8d52cf3eadfe814328f1cff05e3f0022b4cf696ddc8498ef26b52f71b201ad

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
