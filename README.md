# Fafnir

`fafnir` is a command-line utility for developers, written in Rust. It provides a set of tools for managing Git
repositories in bulk and collecting project source code.

This project is a Rust-based migration of the original Bash scripts found in
the [useful-scripts](https://github.com/hazardous-sun/useful-scripts) repository. The goal is to replace the original
scripts with a more robust, performant, and cross-platform compatible tool.

⚠️ **Note:** This is a work in progress, and not all scripts from the original repository have been migrated yet.

---

## Installation

### Prerequisites

1. **Rust:** You must have the [Rust toolchain](https://rustup.rs/) (which includes `cargo`) installed.
2. **Git:** The Git operations commands require `git` to be installed and available in your system's `PATH`.

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

This command compiles the `fafnir` binary and places it in your Cargo bin directory (usually `~/.cargo/bin/`). Make sure
this directory is in your shell's `PATH` to run the command from anywhere.

---

## Usage

You can run `fafnir --help` to see a full list of commands and options.

### Collect Repository Content

Collects the contents of a directory (respecting `.gitignore` and other rules) into a single JSON file. This is useful
for providing context to LLMs.

**Command:** `fafnir collect [OPTIONS]`

**Options:**

* `--output-file <PATH>`: The path to the output file. (Default: `content.json`)
* `--root <PATH>`: The root directory to scan from. (Default: `.`)
* `-i, --ignore <PATH>`: Specific file or directory paths to ignore.
* `--ignore-all <FILENAME>`: File or directory *names* to ignore globally (e.g., `node_modules`).

**Example:**

```bash
# Scan the current directory and save to output.json, ignoring all 'node_modules' folders
fafnir collect --output-file output.json --ignore-all node_modules
```

---

### Batch Git Operations

These commands operate on a list of parent directories. `fafnir` will scan one level deep inside each provided
directory, find all Git repositories, and perform an action.

Before running an action (`pull` or `push`), the tool checks the status of each repository. Actions are only performed
on "clean" repos (no uncommitted changes, upstream is set, and no unpushed commits). A final report is printed
summarizing the status of all other repos.

#### Check Repos

Checks the status of all git repositories and reports on any that have uncommitted changes, no upstream branch, or
unpushed commits.

**Command**: `fafnir check-repos <DIRECTORIES...>`

**Example**:

```bash
# Check all repositories located inside ~/projects and ~/work
fafnir check-repos ~/projects ~/work
```

#### Pull Repos

Pulls the latest changes for all "clean" git repositories.

**Command**: `fafnir pull-repos <DIRECTORIES...>`

**Example**:

```bash
# Pull updates for all clean repos in ~/projects
fafnir pull-repos ~/projects
```

#### Push Repos

Pushes the latest changes for all "clean" git repositories.

**Command**: `fafnir push-repos <DIRECTORIES...>`

**Example**:

```bash
# Push updates for all clean repos in ~/projects
fafnir push-repos ~/projects
```

## Migration Status

This table tracks the migration progress from the
original [useful-scripts](https://github.com/hazardous-sun/useful-scripts) repository.

| Original Script               | Migrated Command     | Status              |                    
|-------------------------------|----------------------|---------------------|
| `collect_repo_content.sh`     | `fafnir collect`     | ✅ Migrated          |
| `check-repos.sh`              | `fafnir check-repos` | ✅ Migrated          |
| `pull-repos.sh`               | `fafnir pull-repos`  | ✅ Migrated          |
| `push-repos.sh`               | `fafnir push-repos`  | ✅ Migrated          |
| `adjust-git-emails.sh`        | -                    | ❌ Not Migrated      |
| `check-history.sh`            | -                    | ❌ Not Migrated      |
| `list-authors.sh`             | -                    | ❌ Not Migrated      |
| `apply-theme.sh`              | -                    | ❌ Not Migrated      |
| `renamefiles.sh`              | -                    | ❌ Not Migrated      |
| `reconnect-device.sh`         | -                    | ❌ Not Migrated      |
| `install.sh` / `uninstall.sh` | `cargo install`      | 📦 Handled by Cargo |
