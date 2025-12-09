FROM rust:1.91.1-bookworm@sha256:c1e5f19e773b7878c3f7a805dd00a495e747acbdc76fb2337a4ebf0418896b33

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
