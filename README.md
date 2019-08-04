Chillup is a tool for searching for dependencies, for use with the [wurstlang.org](Wurst programming language).

Example use:

```
$ chillup --dump | grep animation
https://github.com/Cokemonkey11/WurstUnitAnimations     A wurstscript library that provides extract unit model animation data (indices and durations)
```

Requirements:

- [rust toolchin](https://rustup.rs/) (in particular `cargo`)

Installation:

```
$ cargo install chillup
```
