FROM rust:1.95.0-bookworm@sha256:503651ea31e66ecb74623beabde781059a5978df1595a9e8ed03974d5fec1bf0

WORKDIR /usr/src/app

COPY . .

# RUN cargo build --release

CMD ["/bin/bash"]
# CMD ["cargo", "run", "--release"]
