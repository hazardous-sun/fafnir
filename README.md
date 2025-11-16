# Fafnir

`fafnir` is a command‑line utility for developers (mostly myself), written in Rust. It provides handy tools for
managing Git repositories in bulk and collecting project source code.

---

## Installation

### Prerequisites

1. **Rust:** Install the [Rust toolchain](https://rustup.rs/) (includes `cargo`).
2. **Git:** Git operations require `git` installed and available in your system `PATH`.

### From Source

1. Clone this repository or download the source files.
2. Navigate to the project's root directory:
   ```bash
   cd /path/to/fafnir
   ```
3. Install the binary using Cargo:
   ```bash
   cargo install --path .
   ```

This command compiles the `fafnir` binary and places it in your Cargo bin directory (usually `~/.cargo/bin/`).  
Make sure this directory is in your shell's `PATH` to run the command from anywhere.

---

## Usage

Run `fafnir --help` to see the full list of commands and options.

### Collect Repository Content

Collect the contents of a directory (respecting `.gitignore` and other rules) into a single JSON file. This is useful
for providing context to LLMs.

**Command:** `fafnir collect [OPTIONS] [OUTPUT_FILE] [ROOT]`

**Options:**

- `-i, --ignore <PATH>` — Specific file or directory paths to ignore.
- `--ignore-all <FILENAME>` — File or directory *names* to ignore globally (e.g., `node_modules`).

**Example:**

```bash
# Scan the current directory, ignoring all 'node_modules' folders
fafnir collect --ignore-all node_modules
```

---

### Batch Git Operations

These commands operate on a list of parent directories. `fafnir` scans **one level deep** inside each provided
directory, finds all Git repositories, and performs an action.

Before running an action (`pull` or `push`), the tool checks the status of each repository. Actions are only performed
on **clean** repos (no uncommitted changes, upstream is set, and no unpushed commits). A final report is printed
summarizing the status of all other repos.

#### Check Repos

Check the status of all Git repositories and report any with uncommitted changes, no upstream branch, or
unpushed commits.

**Command:** `fafnir check-repos <DIRECTORIES...>`

**Example:**

```bash
# Check all repositories located inside ~/projects and ~/work
fafnir check-repos ~/projects ~/work
```

#### Pull Repos

Pull the latest changes for all **clean** Git repositories.

**Command:** `fafnir pull-repos <DIRECTORIES...>`

**Example:**

```bash
# Pull updates for all clean repos in ~/projects
fafnir pull-repos ~/projects
```

#### Push Repos

Push the latest changes for all **clean** Git repositories.

**Command:** `fafnir push-repos <DIRECTORIES...>`

**Example:**

```bash
# Push updates for all clean repos in ~/projects
fafnir push-repos ~/projects
```
