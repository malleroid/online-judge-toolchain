FROM rust:1.93.0-bookworm@sha256:d0a4aa3ca2e1088ac0c81690914a0d810f2eee188197034edf366ed010a2b382

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
