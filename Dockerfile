# Latest stable version of Rust as a base image
FROM rust:1.92.0 AS builder

# Swtiching to working directory app
# It will be created if it doesn't exist already
WORKDIR /app 

# System dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all the files from our working environment to our Docker image
COPY . .

# Check offline metadata instead of calling a live db
ENV SQLX_OFFLINE=true

RUN cargo build --release

FROM debian:bullseye-slim AS runtime

WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  # Clean up
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/ghoda ghoda
COPY configuration configuration


ENV APP_ENVIRONMENT=production

# When docker run is executed, launch the binary!
ENTRYPOINT ["./ghoda"]
