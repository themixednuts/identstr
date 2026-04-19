# identstr

Immutable strings for user-supplied identifiers that may or may not be quoted.

`IdentStr` is for identifier strings that come from users: fields, labels, aliases, table names, and similar values. It preserves the way an identifier was quoted while comparing identifiers case-insensitively by default.

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::with_quote("Users", Quote::Double);

assert_eq!(name.as_str(), "Users");
assert_eq!(name.quote(), Some(Quote::Double));
assert_eq!(name, "users");
```

Use `Key` when storing identifiers in maps or sets that are queried repeatedly:

```rust
use std::collections::HashMap;
use identstr::{IdentStr, Key};

let name = IdentStr::new("Users");
let key = Key::from(&name);

let mut columns = HashMap::new();
columns.insert(key, 0);

assert_eq!(columns.get(&Key::new("users")), Some(&0));
```

## Features

- ASCII case-insensitive matching is the default.
- `unicode`: adds Unicode matching modes and Unicode security helpers.

## License

MIT
