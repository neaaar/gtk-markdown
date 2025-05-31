# Usa Debian unstable come base per ottenere versioni aggiornate dei pacchetti
FROM debian:sid

# Imposta variabili di ambiente per non avere richieste interattive
ENV DEBIAN_FRONTEND=noninteractive

# Aggiorna e installa dipendenze base, GTK4, WebKit2GTK e Rust
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

# Installa rustup e Rust stable
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copia i file del progetto nella directory di lavoro
WORKDIR /app
COPY . .

# Compila l'applicazione in release
RUN cargo build --release

# Comando di default per eseguire l'app
CMD ["./target/release/gtk-markdown"]

