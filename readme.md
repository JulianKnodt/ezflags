# ezflags

A simple to use API for command line flags in rust.

Modeled after `golang`'s flag API, it allows for simple no-frills command line parsing.

It's designed to be simple, but doesn't offer the performance or all the features of
[clap](https://github.com/BurntSushi/clap-rs/tree/master/src). If you're looking for high
performance, use clap. This is intended to be a library for building small command line
utilities.

```rust
let mut fs = FlagSet::new();
let mut int_flag: Option<i32> = None;
fs.add("num", "Info about num", &mut int_flag);
// <Binary> -num 3
let _remaining_args = fs.parse_args();
assert_eq!(int_flag, Some(3));
```

It also offers a simple toggle:
```rust
let mut fs = FlagSet::new();
let mut switch = false;
fs.add("switch", "Pass this to set switch", &mut switch);
// <Binary> -switch
let _remaining_args = fs.parse_args();
assert_eq!(switch, true);
```

### Contributions

Any contributions welcome, just leave a pull request, and I will try to get to it.
