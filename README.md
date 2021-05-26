# Advent of Code 2020 in Rust

## Goals
- Gotta go fast
- Somewhat idiomatic rust
- Hopefully no dependencies
- Occasional macro autism
- Learn something new

## Non-goals
- Ridiculously-optimized algorithms
- Overly-verbose real-world-ish types

## Things learned so far
- Would be really cool to have [streaming iterators][1], [`io::Lines`][2] is nice but it allocates a string for each line
- One can accidentally create [a recursive macro][3] that seems to perform better than [itertools' alternative][4]
- [Slice patterns][5] were a thing all this time
- [`mem::swap`][6] and [`mem::replace`][7] come in handy when dealing with `&mut` references

[1]: http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html
[2]: https://doc.rust-lang.org/std/io/struct.Lines.html
[3]: src/utils/macros.rs
[4]: https://docs.rs/itertools/latest/itertools/structs/struct.TupleCombinations.html
[5]: https://blog.thomasheartman.com/posts/feature(slice_patterns)
[6]: https://doc.rust-lang.org/nightly/core/mem/fn.swap.html
[7]: https://doc.rust-lang.org/nightly/core/mem/fn.replace.html
