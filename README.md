Chillup is a tool for searching for dependencies, for use with the [wurst programming language](https://wurstlang.org).

Example use:

```
$ chillup --dump | rg animation
https://github.com/Cokemonkey11/WurstUnitAnimations     A wurstscript library that provides extract unit model animation data (indices and durations)
```

Requirements:

- [rust toolchain](https://rustup.rs/) (in particular `cargo`)

Installation:

```
$ cargo install chillup
```
