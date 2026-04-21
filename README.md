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

Use `from_raw` when the input is already identifier text and should not be
parsed for surrounding quote delimiters.

Use `IdentStr` directly for the normal map API:

```rust
use std::collections::HashMap;
use identstr::{IdentStr, Quote};

let mut tables = HashMap::new();
tables.insert(IdentStr::<Quote>::new("\"Users\""), 0);

assert_eq!(tables.get(&IdentStr::new("users")), Some(&0));

let (stored_name, index) = tables
    .get_key_value(&IdentStr::new("users"))
    .expect("table present");

assert_eq!(stored_name.quote(), Some(Quote::Double));
assert_eq!(*index, 0);
```

Most users can stop there.

`Key` is an optional lookup key for code that already keeps a separate map or
set of canonicalized identifiers:

```rust
use std::collections::HashMap;
use identstr::{IdentStr, Key, Quote};

let mut tables = HashMap::new();
tables.insert(Key::new("\"Users\""), 0);

let lookup = Key::new("\"users\"");
assert_eq!(tables.get(&lookup), Some(&0));
```

## Features

- ASCII case-insensitive matching is the default.
- `unicode`: adds Unicode matching modes and Unicode security helpers.

## License

MIT
