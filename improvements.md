# Rust Code Review: Advent of Code Solutions

## Overall

Your solutions work correctly and show solid algorithmic thinking. The main gap is idiomatic Rust - leveraging the type system and iterator chains to write safer, cleaner code.

---

## Key Patterns to Fix

### 1. Iterator chains over loops

You write a lot of manual loops with mutable state:

```rust
// Day 2 - your version
let mut sum: u128 = 0;
for range_str in ranges {
    let Ok(range) = NumberRange::from_str(range_str) else { continue };
    for i in range.0..=range.1 {
        sum += i as u128;
    }
}
```

Prefer iterator chains:

```rust
// Improved
input
    .split(',')
    .filter_map(|s| s.parse::<NumberRange>().ok())
    .flat_map(|range| range.0..=range.1)
    .map(u128::from)
    .sum()
```

This pattern shows up in every solution. Learn: `filter_map`, `flat_map`, `fold`, `enumerate`.

---

### 2. Type modeling with enums/structs

```rust
// Day 1 - your version
let multiplier: i64 = match chars.next() {
    Some('L') => -1,
    Some('R') => 1,
    _ => panic!("rotations must start with L or R"),
};
```

Make invalid states unrepresentable:

```rust
// Improved
enum Direction { Left, Right }

impl Direction {
    fn multiplier(self) -> i64 {
        match self { Left => -1, Right => 1 }
    }
}
```

The compiler prevents invalid directions. This is Rust's superpower.

---

### 3. Borrowing vs ownership

```rust
// Day 1 - your version
pub fn puzzle1(init_pos: u8, rotations: Vec<String>) -> u64
```

Taking `Vec<String>` forces callers to give up ownership or clone. Better:

```rust
pub fn puzzle1(init_pos: u8, rotations: &[impl AsRef<str>]) -> u64
```

Rule: borrow (`&T`) unless you need to own or mutate.

---

### 4. Safe type conversions

You use `as` everywhere:
```rust
let curr_pos: i64 = init_pos as i64;  // day1
sum += i as u128;                      // day2
let len_i = matrix.len() as i128;     // day4
```

Use `From`/`Into` for lossless conversions:
```rust
let curr_pos = i64::from(init_pos);
```

The `as` operator can silently truncate (`300_u16 as u8 == 44`).

---

### 5. Standard library traits

```rust
// Day 2 - your version
impl NumberRange {
    pub fn from_str(input: &str) -> Result<Self, &str> { ... }
}
// Usage: NumberRange::from_str(s)
```

Implement `FromStr` to get `.parse()` for free:

```rust
impl FromStr for NumberRange {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> { ... }
}
// Usage: s.parse::<NumberRange>()
```

Common traits: `FromStr`, `Display`, `From`, `TryFrom`, `Default`.

---

### 6. Error handling

```rust
// Day 3 - returns Some(0) on parse error!
match format!("{}{}", arr[0], arr[1]).parse::<u128>() {
    Ok(num) => Some(num),
    Err(e) => {
        println!("failed to parse: {}", e);
        Some(0)  // silently corrupts results
    }
}
```

Don't convert errors to default values. Use `Option`/`Result` properly:

```rust
// Just return None, let filter_map handle it
Some(u128::from(result[0]) * 10 + u128::from(result[1]))
```

Also: don't `println!` in production code. And `format!("{}{}", a, b).parse()` is slower than `a * 10 + b`.

---

### 7. Small idioms

| Your code | Idiomatic |
|-----------|-----------|
| `if cond { panic!() }` | `assert!(cond)` |
| `for i in 0..vec.len()` | `for (i, item) in vec.iter().enumerate()` |
| `((x % 100) + 100) % 100` | `x.rem_euclid(100)` |
| `intervals.len() == 0` | `intervals.is_empty()` |
| `split('\n')` | `lines()` |
| `let ref r = x[i]` | `let r = &x[i]` |

---

## Day-Specific Notes

### Day 4: Code duplication
60 lines duplicated between puzzle1/puzzle2. Extract a helper:

```rust
fn count_adjacent(matrix: &[Vec<char>], row: usize, col: usize) -> usize {
    DIRECTIONS.iter()
        .filter(|(dr, dc)| {
            row.checked_add_signed(*dr)
                .zip(col.checked_add_signed(*dc))
                .and_then(|(r, c)| matrix.get(r)?.get(c))
                .is_some_and(|&ch| ch == '@')
        })
        .count()
}
```

Also: mixing `i128` and `usize` with constant `as usize` casts is dangerous. Use `isize` for offsets and `checked_add_signed`.

### Day 5: Good algorithmic thinking
Interval tree implementation shows strong CS fundamentals. Minor issues:

- **`let ref r = intervals[mid]`** → `let r = &intervals[mid]` (day5.rs:97)
  - `ref` in bindings is old style
- **`pub` on private struct fields** → remove `pub` (day5.rs:75-78)
  - `IntervalNode` isn't public, so `pub` does nothing
- **`From` impl panics** → use `TryFrom` (day5.rs:81-88)
  - `From` should never panic per conventions
- **`split('\n').collect()`** → `split_once('-')` (day5.rs:55-57)
  - More efficient for exactly two parts
- **`intervals.len() == 0`** → `intervals.is_empty()` (day5.rs:153)
- **`try_into().expect()`** → `u128::from()` (day5.rs:50)
  - `usize → u128` is infallible on all platforms

The improved version also dereferences range bounds explicitly (`*intervals[0].start()`) which is clearer than relying on auto-deref in variable bindings.

---

## Day 6 Additions

### 8. Extract helper functions to eliminate duplication (DRY)

```rust
// Day 6 - duplicated code in puzzle1 and puzzle2
let mut input_file = File::open(input_path).expect("failed to read input for day 6");
let mut reader = BufReader::new(input_file);
let mut newline_offsets: Vec<u64> = vec![0];
let mut buf = Vec::new();
loop {
    buf.clear();
    let bytes_read = reader.read_until(b'\n', &mut buf).expect("failed to scan input file for newlines");
    if bytes_read == 0 { break; }
    newline_offsets.push(newline_offsets.last().unwrap() + bytes_read as u64);
}
```

Extract to a helper function:

```rust
// Improved
fn read_line_offsets(path: &str) -> (File, Vec<Range<u64>>) {
    let file = File::open(path).expect("failed to open input file");
    let mut reader = BufReader::new(file);
    let mut offsets = vec![0u64];
    let mut buf = Vec::new();

    loop {
        buf.clear();
        let bytes_read = reader.read_until(b'\n', &mut buf)
            .expect("failed to scan input file for newlines");
        if bytes_read == 0 { break; }
        offsets.push(offsets.last().unwrap() + bytes_read as u64);
    }

    let ranges = offsets.windows(2).map(|w| w[0]..w[1]).collect();
    (reader.into_inner(), ranges)
}
```

If code is duplicated between functions, extract it to a helper. This is the DRY (Don't Repeat Yourself) principle.

---

### 9. Use `mem::take` for moving values

```rust
// Day 6 - your version
numbers = vec![];
operation = None;
num_string = String::new();
```

Better with `mem::take`:

```rust
// Improved
let nums = mem::take(&mut numbers);
match operation.take().unwrap() {
    Operation::Add => total_sum += nums.iter().sum::<i64>(),
    Operation::Multiply => total_sum += nums.iter().product::<i64>(),
}
```

`mem::take(&mut x)` moves the value out of `x` and replaces it with `Default::default()`. This is clearer than reassigning empty collections and works with `Option::take()` too.

---

### 10. Local closures for repeated logic

```rust
// Day 6 - your version (repeated 6+ times)
let mut buf = [0u8; 1];
input_file.read_at(&mut buf, char_offset).expect("failed to read the next byte");
let c = buf[0] as char;
```

Extract to a closure:

```rust
// Improved
let read_byte = |offset: u64| -> char {
    let mut buf = [0u8; 1];
    file.read_at(&mut buf, offset).expect("failed to read byte");
    buf[0] as char
};

// Usage
let c = read_byte(offset);
```

Local closures are perfect for repeated logic within a single function, especially when they capture local variables.

---

### 11. Match with pattern guards

```rust
// Day 6 - your version (using unstable let_chains)
if let Some(o) = operation && !numbers.is_empty() {
    match o {
        Operations::Add => { ... },
        Operations::Multiply => { ... }
    }
}
```

Use stable pattern guards:

```rust
// Improved
match operation {
    Some(Operation::Add) if !numbers.is_empty() => {
        total_sum += numbers.iter().sum::<i64>();
    }
    Some(Operation::Multiply) if !numbers.is_empty() => {
        total_sum += numbers.iter().product::<i64>();
    }
    _ => break,
}
```

Pattern guards (`if condition`) in match arms are stable and more idiomatic than unstable `let_chains`.

---

### 12. Small idioms (continued)

| Your code | Idiomatic | Context |
|-----------|-----------|---------|
| `type Operations = ArithmeticEnum` | Just rename to `Operation` | Avoid redundant type aliases |
| `4277556` | `4_277_556` | Numeric literal readability |
| `#[derive(Debug)]` on simple enum | `#[derive(Debug, Clone, Copy)]` | Simple enums should derive Copy |
| `keep_processing` flag | `found_content` | Name what you're checking, not the loop control |

---

## What You're Doing Right

- Solutions work correctly
- Excellent doc comments explaining approach
- Test cases
- Algorithm design (especially the interval tree)
- Past the "fighting the borrow checker" phase

---

## Focus Areas

1. **Master iterators** - Biggest impact on code quality. `filter_map`, `flat_map`, `fold`.
2. **Type-driven design** - Use enums/structs to prevent invalid states.
3. **Borrowing** - Default to `&T` unless you need ownership.

Run `cargo clippy -- -W clippy::pedantic` to catch many of these automatically.

---

## Resources

- [Rust by Example: Iterators](https://doc.rust-lang.org/rust-by-example/trait/iter.html)
- [The Rust Book Ch 13: Functional Features](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
