Making Rust a little more functional.

#How to use:

Put this in your Cargo.toml file:

```
[dependencies]
functils = "0.0.1"
```

Then import with:

```
extern crate functils
use functils::*
```

It's recommended that the above code block be put into your crate root
so you can use the methods everywhere without needing to import it in
each file.
