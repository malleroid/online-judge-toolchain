FROM rust:1.93.0-bookworm@sha256:812df42b4a866cf7165934691a0a89061281679a145b857dc679be8132e709b9

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
