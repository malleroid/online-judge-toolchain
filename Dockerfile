FROM rust:1.92.0-bookworm@sha256:9676d0547a259997add8f5924eb6b959c589ed39055338e23b99aba7958d6d31

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
