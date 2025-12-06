FROM rust:1.91.1-bookworm@sha256:8fed34f697cc63b2c9bb92233b4c078667786834d94dd51880cd0184285eefcf

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
