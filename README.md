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

If the same identifier is looked up repeatedly, cache a `Key` and keep the
original identifier in the value:

```rust
use std::collections::HashMap;
use identstr::{IdentStr, Key, Quote};

let table = IdentStr::<Quote>::new("\"Users\"");

let mut tables = HashMap::new();
tables.insert(table.to_key(), (table, 0));

let lookup = Key::new("users");
let (stored_name, index) = tables.get(&lookup).expect("table present");

assert_eq!(stored_name.quote(), Some(Quote::Double));
assert_eq!(*index, 0);
```

## Features

- ASCII case-insensitive matching is the default.
- `unicode`: adds Unicode matching modes and Unicode security helpers.

## License

MIT
