FROM rust:1.94.1-bookworm@sha256:1685eb7dd8d23b3eed9c95094b9f590487e25ae46bab8f76ea488c4000404322

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
