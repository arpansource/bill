# 🛠️ Build Guide

This document describes how to build the `bill` CLI across supported platforms.

---

## 📦 Prerequisites

* Rust (latest stable)
* Cargo
* Docker (recommended for cross-platform builds)

---

## 🧱 Local Build (macOS / Linux)

```bash
cargo build --release
```

Output:

```
target/release/bill
```

---

## 🍎 macOS (Apple Silicon / Intel)

```bash
cargo build --release
```

Verify:

```bash
file target/release/bill
```

---

## 🐧 Linux Build (x86_64)

### Recommended: Docker (reproducible)

```bash
docker run --rm -it \
  --platform linux/amd64 \
  -v "$PWD":/app \
  -w /app \
  rust:latest \
  bash
```

Inside container:

```bash
cargo build --release
```

Output:

```
target/release/bill
```

Verify:

```bash
file target/release/bill
# Expected: ELF 64-bit LSB executable, x86-64
```

---

## 🐧 Linux ARM (optional)

Build on an ARM machine or ARM container:

```bash
cargo build --release
```

---

## 🪟 Windows Build (.exe)

### Using Docker

```bash
docker run --rm -it \
  --platform linux/amd64 \
  -v "$PWD":/app \
  -w /app \
  rust:latest \
  bash
```

Inside container:

```bash
apt update
apt install -y mingw-w64

rustup target add x86_64-pc-windows-gnu

cargo build --release --target x86_64-pc-windows-gnu
```

Output:

```
target/x86_64-pc-windows-gnu/release/bill.exe
```

Verify:

```bash
file target/x86_64-pc-windows-gnu/release/bill.exe
# Expected: PE32+ executable (console) x86-64
```

---

## ⚠️ SQLite Dependency

This project uses:

```
rusqlite = { version = "...", features = ["bundled"] }
```

This ensures:

* SQLite is compiled and bundled
* No system dependency is required
* Cross-platform builds succeed

---

## 📦 Packaging

### macOS

```bash
zip bill-macos.zip target/release/bill
```

---

### Linux

```bash
zip bill-linux-x86_64.zip target/release/bill
```

---

### Windows

```bash
zip bill-windows.zip target/x86_64-pc-windows-gnu/release/bill.exe
```

---

## 🚀 Release Checklist

* [ ] Build macOS binary
* [ ] Build Linux x86_64 binary
* [ ] Build Windows binary
* [ ] Verify binaries using `file`
* [ ] Package binaries (zip)
* [ ] Upload to GitHub Releases

---

## 🔧 Troubleshooting

### Error: `cannot find -lsqlite3`

Fix:

* Ensure `rusqlite` uses the `bundled` feature

---

### Error: `unrecognized command-line option '-m64'`

Cause:

* Architecture mismatch (ARM vs x86)

Fix:

* Use Docker with `--platform linux/amd64`

---

### Windows build fails

Ensure:

```bash
apt install -y mingw-w64
```

---

## 📌 Summary

| Platform     | Method                 |
| ------------ | ---------------------- |
| macOS        | native build           |
| Linux x86_64 | Docker (`linux/amd64`) |
| Windows      | Docker + mingw         |

---

This setup ensures consistent, reproducible builds across platforms.
