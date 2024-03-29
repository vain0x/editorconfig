# EditorConfig templates

Tiny script to generate `.editorconfig`.

## Prerequisites

- curl

## Install

Install cargo from <https://rustlang.org>.

```sh
cargo install editorconfig --git https://github.com/vain0x/editorconfig --branch main
```

## Usage

To generate `.editorconfig` for Rust project, do:

```sh
editorconfig rust
```

and `.editorconfig` file is created or overwritten with:

```conf
root = true

[*.rs]
indent_size = 4
insert_final_newline = true
trim_trailing_whitespace = true
```

### Multiple languages

Split language names by comma `,`:

```sh
editorconfig 'html,css,ts'
```

### List of Languages

See [templates/index.txt](templates/index.txt).
