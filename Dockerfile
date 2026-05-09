FROM rust:1.95.0-bookworm@sha256:e99e99e1d37512e741713d282b65b6452385f7b57d1a86cbceffa2a3cc128970

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
