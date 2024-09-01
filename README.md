# Lisper

- A library used for tokenizing and parsing lists in the style of Lisp
- Meant to be embedded
- Not a scripting language but can easily be used to build one

## Installation

Add to your `Cargo.toml` file dependencies:

```toml
[dependencies]
lisper = { git = "https://github.com/ericrobolson/Lisper.git" }
```

## Example Usage

```

let contents = "(def (add-multiply a b c)
    (* c (+ a b)))";

// let nodes = lisper::parse_file(contents, path)?;
let mut nodes = lisper::parse_str(contents)?;
let id = nodes[0].pop_identifier("identifier").unwrap();
```
