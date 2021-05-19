# Advent of Code 2020 in Rust

## Goals
- Gotta go fast — prefer stack over heap, iterators over loops, etc.
- Somewhat idiomatic rust
- Hopefully no dependencies
- Occasional macro autism
- Learn something new

## Non-goals
- Ridiculously-optimized algorithms
- Overly-verbose real-world-ish types

## Things learned so far
- Would be really cool to have [streaming iterators][1], [std::io::Lines][2] is nice but it allocates a new string for each line
- One can accidentally create [a recursive macro][3] that seems to perform better than [itertools' alternative][4]
- [Slice patterns][5] were a thing all this time

[1]: http://lukaskalbertodt.github.io/2018/08/03/solving-the-generalized-streaming-iterator-problem-without-gats.html
[2]: https://doc.rust-lang.org/std/io/struct.Lines.html
[3]: src/day01/macros.rs
[4]: https://docs.rs/itertools/latest/itertools/structs/struct.TupleCombinations.html
[5]: https://blog.thomasheartman.com/posts/feature(slice_patterns)
