# Lisper

- A library used for tokenizing and parsing lists in the style of Lisp
- Meant to be embedded
- Not a scripting language but can easily be used to build one

# Example Usage

```

let contents = "(def (add-multiply a b c)
    (* c (+ a b)))";

let nodes = lisper::parse(contents, path)?;

let list = nodes.as_list();
let def = list.get(0).unwrap().as_identifier();

```
