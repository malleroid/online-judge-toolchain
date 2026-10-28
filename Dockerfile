FROM rust:1.99.1-bookworm@sha256:e9da6f6c3ed5095fd5d33bf7cdf309830a178a8147888f1b2f76d5769bd9306c

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
