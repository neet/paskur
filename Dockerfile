FROM rust:1.66 as builder
WORKDIR /app

COPY . .
RUN cargo install --path .
RUN cargo build

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