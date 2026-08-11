FROM rust:1.97.1-bookworm@sha256:593db038d36ea0d2e5b22a2e095eb95dbec607d88d658c2f35f65ee880b4f93a

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
