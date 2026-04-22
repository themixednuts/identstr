# identstr

Immutable identifier strings for user input that may or may not be quoted.

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::new("\"Users\"");

assert_eq!(name.as_str(), "Users");
assert_eq!(name.quote(), Some(Quote::Double));
assert_eq!(name, "users");
assert_eq!(name.to_quoted_string(), "\"Users\"");
```

If you already split the quote:

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::with_quote("Users", '"');

assert_eq!(name.quote(), Some(Quote::Double));
```

Use `from_raw` to skip quote parsing.

`Display` writes the identifier text without quote delimiters. Use
`to_quoted_string` or `write_quoted` when you need the preserved quote style.

## Features

- ASCII case-insensitive matching is the default.
- `unicode`: adds Unicode matching modes and Unicode security helpers.

## License

MIT
