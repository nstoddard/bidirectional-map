A two-way map data structure for cloneable keys and values.

This is currently pretty minimal; it doesn't yet support iterators, for instance.

Most functions come in `_fwd` and `_rev` variants; where the `_fwd` varaint acts on the second
entry given the first, and `_rev` is the opposite.

This crate is best for values that are cheap to clone, since internally it stores two copies
of each element. To use it with large values, consider wrapping them in `Rc` to make them cheap
to clone.
