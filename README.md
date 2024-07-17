# Rust Learning

## Steps

1. Visit [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
2. [Install Rust](https://www.rust-lang.org/tools/install) locally
3. Install the [Rust VSCode extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   For the extension to work, we need to open a project in a separate VS Code window.
4. Using `cargo new your-project` to initialize a project.

## [hello-world](./hello-world/)

Interesting learning: [macro_rules](https://doc.rust-lang.org/rust-by-example/macros.html)

## [primitives](./primitives/)

Interesting learning: Variables are not mutable by default.

## [custom-types](./custom-types/)

Interesting learnings:

- [String vs str](https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str)
- `struct`: define a structure
  - Tuple structs, which are, basically, named tuples.
  - The classic C structs
  - Unit structs, which are field-less, are useful for generics.
- `enum`: define an enumeration
