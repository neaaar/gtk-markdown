# Use Debian unstable as base to get up-to-date packages
FROM debian:sid

# Set environment variables to avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Update and install base dependencies, GTK4, WebKit2GTK, and Rust
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libgtk-4-dev \
    libwebkit2gtk-4.1-dev \
    libglib2.0-dev \
    libpango1.0-dev \
    libgdk-pixbuf-2.0-dev \
    libxml2-dev \
    ca-certificates \
    git \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Install rustup and stable Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy the project files into the working directory
WORKDIR /app
COPY . .

# Build the application in release mode
RUN cargo build --release

# Default command to run the app
CMD ["./target/release/gtk-markdown"]
