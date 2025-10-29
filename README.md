## animal-age

`animal-age` is a colorful Rust CLI that converts pet ages into their human-year equivalents, highlights lifespan progress, and outputs either terminal-friendly bars or machine-readable JSON.

### Highlights
- Converts 11 supported animal types (cat, several dog sizes, rabbit, etc.) with species-specific formulas.
- Renders progress bars that compare the pet’s lifespan to an 80-year human baseline, with optional color suppression for plain terminals.
- Suggests close matches when a typo is detected in the `--type` flag (Levenshtein distance).
- Accepts comma-separated animal lists so you can compare multiple pets in a single run.
- Emits warnings when the supplied age exceeds 150 % of the expected lifespan.
- Supports structured output via `--json` for piping into scripts, dashboards, or spreadsheets.

## Installation

```bash
cargo install --path .
```

The binary targets Rust 1.70+ (Edition 2021). If you only want to try it locally, running `cargo run -- <args>` works as well.

## Usage

```
animal-age [OPTIONS] --type <ANIMAL>[,<ANIMAL>...] --age <YEARS>
```

| Flag | Description |
| --- | --- |
| `-t`, `--type` | Animal type; use `--list` to view valid keys. Accepts comma-separated values or repeated flags. |
| `-a`, `--age` | Real age in years (floating point). Must be non-negative. |
| `--list` | Print supported animals and exit. |
| `--json` | Emit JSON rows instead of bar charts. |
| `--no-color` | Disable ANSI coloring (handy for logs or monochrome terminals). |

### Examples

List the available animal keys:

```bash
animal-age --list
```

Convert one pet and show colored bars:

```bash
animal-age --type cat --age 3
```

Compare multiple pets and export JSON:

```bash
animal-age -t cat,small_dog -a 3 --json
```

Or repeat the `-t` flag to compare multiple animals at the same age:

```bash
animal-age -a 3 -t cat -t small_dog
```

Sample JSON payload:

```json
{
  "animal": "cat",
  "age": 3.0,
  "human_age": 29.0,
  "animal_max_lifespan": 18.0,
  "human_max_lifespan": 80.0,
  "animal_progress": 0.16666667,
  "human_progress": 0.3625
}
```

## Supported Animals

Key | Description | Typical Max Age
--- | --- | ---
`small_dog` | Small dog (e.g., terrier) | 16 years
`medium_dog` | Medium dog (e.g., spaniel) | 14 years
`big_dog` | Large dog (e.g., retriever) | 10 years
`cat` | Domestic cat | 18 years
`horse` | Horse | 30 years
`pig` | Pig | 20 years
`parakeet` | Parakeet / budgie | 10 years
`snake` | Common pet snake | 20 years
`goldfish` | Goldfish | 15 years
`rabbit` | Rabbit | 12 years
`hamster` | Hamster | 3 years

## Development

```bash
cargo fmt          # format (if needed)
cargo clippy       # lint
cargo test         # run unit tests
```

The existing tests focus on conversion math, but feel free to add more coverage around lifespan bars or CLI parsing as you extend the tool. Contributions are welcome under the MIT license (see `LICENSE`).
