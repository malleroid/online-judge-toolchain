FROM rust:1.94.0-bookworm@sha256:6a544e5d08298a8cddfe9e7d3b4796e746601d933f3b40b3cccc7acdfcd66e0d

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
