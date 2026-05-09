FROM rust:1.95.0-bookworm@sha256:84696f49efb05ce30737a421bf5637fb0d553799aef0169fa3657a68210344b0

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
