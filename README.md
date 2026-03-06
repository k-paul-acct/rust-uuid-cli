# UUID Generator

A fast, flexible UUID generator for the command line, written in Rust.

### Features

- **Multiple versions** – generate UUID v4 (random) or v7 (time-ordered)
- **Flexible formats** – supports `d`, `n`, `b`, `p`, and `x` output formats
- **Case control** – use uppercase letters (e.g. `D` vs `d`) for uppercase or lowercase output
- **Bulk generation** – generate any number of UUIDs in a single command
- **Zero dependencies at runtime** – single static binary

### Build & Run

```bash
# clone / copy the project, then:
cargo build --release

# generate a single UUID (defaults to UUID v4, format d)
./target/release/uuid

# generate 5 UUIDs
./target/release/uuid 5

# generate UUID v7
./target/release/uuid -v7

# generate in uppercase N format
./target/release/uuid -fN

# combine options: 3x UUID v7 in uppercase braced format
./target/release/uuid 3 -v7 -fB

# additional information
./target/release/uuid --help
```

### Output Formats

| Flag | Example output |
|------|----------------|
| `d` or `D` | `e3d3d7a1-daaf-498c-b165-29c277a33b23` |
| `n` or `N` | `1c3535bea2594ce2982ad7eae33a76bd` |
| `b` or `B` | `{d4b1d42c-587f-45f2-b431-7bdd3da4b90c}` |
| `p` or `P` | `(f2820ce6-784f-4b07-842b-c581e303e2a9)` |
| `x` or `X` | `{0xc9f3744c,0x853f,0x4f51,{0xa1,0x0c,0x9c,0xbd,0x36,0x82,0xeb,0xc3}}` |

Lowercase flag letters produce lowercase hex output; uppercase flag letters produce uppercase hex output.

### Configuration

1. To show all available options, use the `--help` flag.
