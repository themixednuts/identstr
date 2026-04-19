# identstr

Immutable strings for user-supplied identifiers that may or may not be quoted.

`IdentStr` stores the unquoted text a user wrote, keeps optional quote metadata, and compares identifiers with a policy. The default policy is ASCII case-insensitive, matching common SQL identifier behavior.

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

- `unicode`: enables Unicode comparison policies and security helpers.

## License

MIT
