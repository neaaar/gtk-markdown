# Use Debian unstable for up-to-date packages
FROM debian:sid

# Avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install GTK‑4, WebKitGTK‑6, Rust, and build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libgtk-4-dev \
    libwebkitgtk-6.0-dev \
    libglib2.0-dev \
    libpango1.0-dev \
    libgdk-pixbuf-2.0-dev \
    libxml2-dev \
    ca-certificates \
    git \
    pkg-config \
    libssl-dev \
  && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /app
COPY . .

RUN cargo build --release

CMD ["bash"]

