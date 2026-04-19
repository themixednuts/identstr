# identstr

Immutable identifier strings with preserved quote style and configurable comparison policy.

`IdentStr` stores the unquoted text a user wrote, keeps optional quote metadata, and compares identifiers with a policy. The default policy is ASCII case-insensitive, matching common SQL identifier behavior.

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::with_quote("Users", Quote::Double);

assert_eq!(name.as_str(), "Users");
assert_eq!(name.quote(), Some(Quote::Double));
assert_eq!(name, "users");
```

For repeated lookups, use a cached key:

```rust
use identstr::{IdentStr, Key};

let name = IdentStr::new("Users");
let key = Key::from(&name);

assert_eq!(key.as_str(), "users");
```

## Features

- `unicode`: enables Unicode comparison policies and security helpers.

## License

MIT
