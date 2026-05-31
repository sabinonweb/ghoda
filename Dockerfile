FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 as chef
WORKDIR /app 

# System dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Latest stable version of Rust as a base image
FROM chef as planner

# Swtiching to working directory app
# It will be created if it doesn't exist already

# Copy all the files from our working environment to our Docker image
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

From chef as builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo-chef cook --release --recipe-path recipe.json
# If recipe.json is same, all layers are cached 
COPY ..

# Check offline metadata instead of calling a live db
ENV SQLX_OFFLINE=true

RUN cargo build --release --bin ghoda

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
