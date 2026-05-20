
# Async Directory Crawler

A lightweight, asynchronous Rust tool built on top of Tokio and Serde that recursively crawls a target directory, extracts file contents, and mirrors the entire folder hierarchy into a structured `crawl.json` file.

## Features

- **Asynchronous I/O**: Leveraging `tokio::fs` to read directories and file contents concurrently without blocking execution.
- **Strict Tree Layout**: Replicates folder hierarchies accurately by using nested JSON maps.
- **Directory Formatting**: Automatically appends a trailing slash (`/`) to folder names to visually distinguish them from files inside the output file.

---

## Output Example

Running this tool will generate a `crawl.json` file formatted like this:

```json
{
  "src/": {
    "main.rs": "use serde_json::{Map, Value};\n...",
    "utils/": {
      "helpers.rs": "// helper functions go here"
    }
  },
  "Cargo.toml": "[package]\nname = \"crawler\"..."
}

```

---

## Getting Started

### 1. Prerequisites

Ensure you have [Rust and Cargo installed](https://www.rust-lang.org/tools/install).

### 2. Installation

Run this in cloned repository:

```bash
cargo install --path .
```

### 3. Usage

```bash
\<Some(path)> > fcrawl
```
Upon success, you will see the console confirmation:

```text
Crawling directory structure...
Successfully saved directory tree structure to crawl.json!
```

You can now check your project root for the newly created `crawl.json` file.

---


---

## Edge Case Considerations

* **Binary / Image Files**: The crawler relies on `fs::read_to_string`. If it encounters a non-UTF8 binary file (like an image or an executable), it will safely skip over reading its contents to avoid crashing.
* **Symlinks**: Circular reference boundaries are limited to how `fs::read_dir` natively resolves paths. To prevent infinite loops in heavily nested environments with internal symlinks, consider vetting input directories beforehand.

```

```
