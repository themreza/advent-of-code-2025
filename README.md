# Advent of Code 2025 (Rust)

[Advent of Code](https://adventofcode.com/) is a series of programming challenges released every year throughout December.

The challenges have a Christmas theme and consist of stories of the Elves based in the North Pole. This year's mission is to help the Elves finish decorating the North Pole.

While there are usually 25 challenges, this year's Advent of Code ends on December 12. It also does not feature a global leaderboard due to AI misuse.

My solutions are in the `src` directory. You can run them via `make run <binary name, e.g. day1>`, run the tests with `make tests`, and read the documentation with `make docs`.

The solutions are not perfect or fully optimized. I gave each challenge some thought, but I mainly did this to get more comfortable with Rust. But just because it's Rust, it should be fast, right? 😛

## Rust Resources

- [Rust Design Patterns](https://rust-unofficial.github.io/patterns/)
- [Exploring Rust as a Go Developer](https://zerotohero.dev/roadmap/learning-rust/)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
- [Clippy linter](https://github.com/rust-lang/rust-clippy)

## Algorithm Resources

- [VisuAlgo - Visualized Algorithms](https://visualgo.net)

## AI Disclaimer

I wrote the solutions myself as the primary objective was learning Rust and just having fun.

After submitting the solution, I then asked for code review from AI to figure out if there are more idiomatic and efficient approaches. I've included both versions, with the improved version in a 
separate file with the `-improved.rs` suffix.

I used Claude 4.5 Opus with the following prompt to generate the improved version:

> I'm learning Rust and I've written this code. Improve the code to make it as idiomatic, clean, easy to read,
> and efficient as possible while not changing the algorithms. Include a summary of changes under the "Day ..."
> inner doc comment with a Markdown format.

## Author

Challenges written by [Eric Wastl (Advent of Code)](https://adventofcode.com/2025/about).

Solutions written by [Mohammad Tomaraei](https://www.linkedin.com/in/tomaraei/)

![](assets/logos/mt.png)

