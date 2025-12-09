FROM rust:1.91.1-bookworm@sha256:6bcbf6ab74b99794ed874fbfa7b06fde7880493fc488ffbec36bf4c1001c94f5

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
