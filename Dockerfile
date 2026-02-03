FROM rust:1.93.0-bookworm@sha256:5c836835420f12753291d480669627d300f92c51bcd65f98a7439d607e978c37

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
