FROM rust:1.66 as builder
WORKDIR /app

# Just copy the manifests
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to build dependencies
RUN mkdir src \
 && echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs \
 && cargo build --release \
 && rm -rf src/

# Now copy the rest of the application
COPY . .
RUN cargo install --path .

# production stage
FROM debian:bullseye-slim

WORKDIR /app
ENV PORT="8080"

RUN apt-get update \
 && apt-get install -y ca-certificates libssl-dev \
 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/paskur /usr/local/bin/
RUN chmod +x /usr/local/bin/paskur

EXPOSE ${PORT}

CMD ["paskur"]
