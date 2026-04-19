# identstr

Immutable identifier strings for user input that may or may not be quoted.

Use `IdentStr` for database input, config keys, schema fields, DSL symbols, command names, or other user-defined identifiers where the original quoting matters but matching should stay case-insensitive by default.

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::new("\"Users\"");

assert_eq!(name.as_str(), "Users");
assert_eq!(name.quote(), Some(Quote::Double));
assert_eq!(name, "users");
```

Use `with_quote` when the quote has already been split from the identifier:

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::with_quote("Users", '"');

assert_eq!(name.quote(), Some(Quote::Double));
```

Use `Key` for repeated map and set lookups:

```rust
use std::collections::HashMap;
use identstr::{IdentStr, Key};

let table = IdentStr::new("\"Users\"");

let mut tables = HashMap::new();
tables.insert(table.to_key(), 0);

assert_eq!(tables.get(&Key::new("users")), Some(&0));
```

## Features

- ASCII case-insensitive matching is the default.
- `unicode`: adds Unicode matching modes and Unicode security helpers.

## License

MIT
