Making Rust a little more functional.

[![Build Status](https://travis-ci.org/mgattozzi/functils.svg?branch=master)](https://travis-ci.org/mgattozzi/functils)
[![Dependency Status](https://dependencyci.com/github/mgattozzi/functils/badge)](https://dependencyci.com/github/mgattozzi/functils)
[![codecov](https://codecov.io/gh/mgattozzi/functils/branch/master/graph/badge.svg)](https://codecov.io/gh/mgattozzi/functils)
[![crates.io](https://img.shields.io/crates/v/functils.svg)](https://crates.io/crates/functils)

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
