FROM rust:1.94.1-bookworm@sha256:6ae102bdbf528294bc79ad6e1fae682f6f7c2a6e6621506ba959f9685b308a55

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
