# Latest stable version of Rust as a base image
FROM rust:1.92.0

# Swtiching to working directory app
# It will be created if it doesn't exist already
WORKDIR /app 

# System dependencies for our linking configuration
RUN apt update && apt install lld clang -y

# Copy all the files from our working environment to our Docker image
COPY . .

# Check offline metadata instead of calling a live db
ENV SQLX_OFFLINE true

RUN cargo build --release

# When docker run is executed, launch the binary!
ENTRYPOINT ["./target/release/ghoda"]
