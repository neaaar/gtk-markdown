# GTK Markdown

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Build with Docker](https://img.shields.io/badge/Docker-supported-blue.svg)](https://www.docker.com/)
[![GTK](https://img.shields.io/badge/GTK-4%20|%206-green.svg)](https://www.gtk.org/)
[![Rust](https://img.shields.io/badge/Rust-stable-orange.svg)](https://www.rust-lang.org/)

GTK Markdown is a lightweight **Markdown editor** written in **Rust** and **GTK4**, 
featuring a real-time preview powered by **WebKitGTK** and **Highlight.js** for syntax highlighting.

The project is currently under development and runs inside a Docker container.

## Features
- Text editor with Markdown support
- Real-time document preview
- Syntax highlighting in code blocks using [highlight.js](https://highlightjs.org/)
- GTK4 interface consistent with GNOME design

---

## Requirements
- [Docker](https://docs.docker.com/get-docker/) installed on the host system
- A Linux system with **X11** support (required to display the GUI from inside the container)
  - On Wayland, you can run it via XWayland

---

## How to run

### Clone the repository
```bash
git clone https://github.com/neaaar/gtk-markdown.git
cd gtk-markdown
```

### Build the Docker image
Run the following command while inside the project folder:
```bash
docker build -t gtk-markdown .
```

### Start the container
Run the included script:
```bash
./start-container.sh
```
This mounts the project folder into `/app` inside the container and starts an interactive session as `root`.

### Start the application
Inside the container, run:
```bash
cargo run --release
```
The application will be compiled and launched.

You should see the GTK Markdown editor window appear on your host desktop.

---

## Licenses and third-party software
- **GTK Markdown** is distributed under the MIT license (see [LICENSE](LICENSE)).
- Uses [highlight.js](https://github.com/highlightjs/highlight.js) (BSD-3-Clause).
- Includes the GitHub highlight.js CSS theme (BSD-3-Clause).

For details, see [NOTICE.md](NOTICE.md).

---

This README.md file was entirely coded and previewed using GTK Markdown.
