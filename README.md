# Crab Compare: A branch Comparison Tool

Crab Compare is a Rust-based CLI tool designed to streamline the process of comparing Git branches. It provides
developers with the ability
to quickly assess whether two branches are identical, facilitating seamless integration and development workflows.
Utilizing Rust's performance capabilities, Crab Compare
ensures fast, accurate, and resource-efficient comparisons.

## Key Features:

- Branch Comparison: Compares one branch against all other within the same repository.
- Efficient and Fast: utilizes Rust's performance to quickly compare branches.
- Simple CLI: easy to use command-line interface
- Configurable via TOML: Supports configuration through a toml file for easy setup and repeat usage.

### Installation (WIP):

#### Prerequisites:

before you being, ensure you meet the following requirements:

- Rust (latest stable version recommended)
- Cargo (comes with rust 🦀)
- Git

#### Installation:

##### Build from source:

```bash
git clone git@github.com:nicolas-moon/crab_compare.git
cd crab_compare
cargo build --release
```

##### Install from source:

```bash 
git clone git@github.com:nicolas-moon/crab_compare.git
cd crab_compare
cargo install --path .
```

## Configuration:

Crab Compare can be configured using a `crab.toml` file located in the current directory, or by specifying options
directly
via command-line arguemtns An example configuration file might look like this:

```toml
repo_path = "/path/to/repo"
target_branches = ["main", "develop", "feature/branch"]
output_format = "Markdown" # or "Table"
```

## Usage:

Run Crab Compare by using the following command:

From build directory:

```bash
./target/release/crab_compare
```

Using install command:

```bash
crab_compare
```

you can override the TOML configuration with command-line arguments:

```bash
crab_compare --repo-path "/another/path" --target-branches main,dev --ouput-format Table
```

## Command-Line Options

- `--config`: Path to the configuration file. Default is `crab.toml`
- `--repo-path`: Path to the Git repository
- `--target-branches`: Comma-separated list of branches to compare
- `--ouput-format`: Output format (Markdown or Table)

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any
contributions
you make are **greatly appreciated**.

if you have a suggestion that would make this better, please for the repo and create a pull request. You can also simply
open an issue
with the tag  `enhancement`.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the BSD 3-Clause License - see the `LICENSE` file for details.

## Contact

Project Link: https://github.com/nicolas-moon/crab-compare


