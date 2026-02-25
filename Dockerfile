FROM rust:1.93.1-bookworm@sha256:52abdd7111dc410daf146085246ffa4b59ccecefc8b2b702e1aab6f06d6abdcf

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
