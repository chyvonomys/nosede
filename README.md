# nosede
a `serde` replacement, stub that does nothing

# why
This can be useful if you have many items that derive serde `Serialize`/`Deserialize` but at some point you decided to disable this functionality and exclude serde crates from dependencies (or conditionally use them, e.g. using cargo `features`)

In this case instead of removing all `Serialize` `Deserialize` derives and possible field attributes (like `#[serde(skip_serializing_if ...)]`) you just use this crate which exposes all this features but they are empty.
