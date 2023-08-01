# Neon - Rust core with TypeScript scripting

Contents:

- [cpu-count](cpu-count/README.md) is the example from the Neon docs.
- [allbooks](allbooks/README.md) is a more complex example with more complex Rust types.

## Example

Build the Rust code with `cargo build` from the root and the `allbooks` project from its
directory with `npm install` and `npm run build --release`.

You can now use it from the root with `node`:

```JavaScript
const allbooks = require('./allbooks')
allbooks.get_books()
```

Output:

```JavaScript
[
  {
    title: 'Chadwick the Crab',
[
  {
    title: 'Chadwick the Crab',
    author: 'Priscilla Cummings',
    year: 2009
  },
  {
    title: 'The Little Prince',
    author: 'Antoine de Saint-Exupéry',
    year: 1943
  },
  { title: 'The Hobbit', author: 'J. R. R. Tolkien', year: 1937 }
]
```

## Requirements

- NPM 7 minimum
- Rust 