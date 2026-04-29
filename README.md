# identstr

Immutable identifier strings for quoted or unquoted names.

```rust
use identstr::{IdentStr, Quote};

let name = IdentStr::new("\"Users\"");

assert_eq!(name.as_str(), "Users");
assert_eq!(name.quote(), Some(Quote::Double));
assert_eq!(name, "users");
assert_eq!(name.to_quoted_string(), "\"Users\"");
```

Use `IdentStr::new` for source text that may include quote delimiters. Use
`IdentStr::from_raw` when the value is already identifier text and should not be
parsed for quotes.

```rust
use identstr::{IdentStr, Quote};

let parsed = IdentStr::new("\"Users\"");
let raw = IdentStr::from_raw("\"Users\"");
let already_split = IdentStr::with_quote("Users", '"');

assert_eq!(parsed.as_str(), "Users");
assert_eq!(raw.as_str(), "\"Users\"");
assert_eq!(already_split.quote(), Some(Quote::Double));
```

`Display` writes the identifier text without quote delimiters. Use
`to_quoted_string`, `write_quoted`, or `display_quoted` when you need the
preserved quote style.

## Keys

`IdentStr` keeps the original spelling and quote style. Use `Key` when a map
only needs normalized lookup text.

```rust
use std::collections::HashMap;

use identstr::{Key, policy};

let mut tables = HashMap::new();
tables.insert(Key::<policy::Ascii>::new("\"Users\""), 7);

assert_eq!(tables.get(&Key::new("users")), Some(&7));
assert_eq!(tables.get(&Key::new("\"users\"")), Some(&7));
```

## Input Ownership

Constructors accept borrowed text and common owned string types. Owned inputs are
reused when the selected storage type can keep them without copying.

```rust
use identstr::{ArcStorage, IdentStr, Quote, policy};
use std::sync::Arc;

let boxed = IdentStr::<Quote>::new(Box::<str>::from("customer_id"));
let shared = IdentStr::<Quote, policy::Ascii, ArcStorage>::new(Arc::<str>::from("customer_id"));

assert_eq!(boxed, shared);
```

## Features

- ASCII case-insensitive matching is available without optional features.
- `unicode`: adds Unicode matching modes and Unicode security helpers.

## License

MIT
