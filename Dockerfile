FROM rust:1.98.1-bookworm@sha256:9a73a5088750b4c95158ab26629c854c3d6fc4b173cb7bc8079ad252d8ed7bfa

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
