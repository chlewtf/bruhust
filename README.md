<div align="center">

# Bruhust

### *The Gen Z Programming Language*

[![CI](https://github.com/chlewtf/bruhust/actions/workflows/ci.yml/badge.svg)](https://github.com/chlewtf/bruhust/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/built%20with-Rust-orange?logo=rust)](https://www.rust-lang.org/)

> *"No cap, this hits different fr fr"*

Bruhust is an open-source toy programming language written in Rust.  
All syntax is Gen Z slang. Files use the `.bruh` extension.  
Absolutely slay.

</div>

---

## Quick Start

```bash
git clone https://github.com/chlewtf/bruhust
cd bruhust
cargo build --release
./target/release/bruhust examples/hello.bruh
./target/release/bruhust --repl       # interactive REPL
```

---

## Syntax Reference

### Print
```
yeet "hello world"            # println
yeet_raw "no newline"         # print (no \n)
```

### Variables
```
no_cap x be 42                # immutable  (let)
lowkey count be 0             # mutable    (var)
hits_diff count be count + 1  # reassign
```

### Booleans & Null
```
bussin       # true
mid          # false
understood   # null
```

### Conditionals
```
fr_fr x > 10 {
    yeet "slaying"
} nah {
    yeet "mid energy"
}
```

### While Loop
```
lowkey i be 0
slay i < 5 {
    yeet i
    hits_diff i be i + 1
}
```

### Loop Control
```
ghosted    # break
periodt    # continue
```

### Functions
```
sus add(a, b) {
    bet a + b         # return
}
yeet rizz add(3, 4)   # call → 7
```

### Arrays
```
lowkey vibes be sus_list ["fire", "bussin", "slay"]
no_cap arr be [1, 2, 3]        # shorthand syntax
yeet drip vibes[0]             # index → "fire"
glow_up vibes "periodt"        # push
yeet no_thoughts vibes         # length
```

### Range
```
no_cap nums be sheesh(0, 5)    # [0, 1, 2, 3, 4]
```

### Match / Switch
```
vibe_check mood {
    facts: "slay" => {
        yeet "you're eating 👑"
    }
    facts: "mid" => {
        yeet "meh"
    }
    cap: => {
        yeet "idk bestie"
    }
}
```

### Type Conversions
```
based "42"      # → number 42
vibe 42         # → string "42"
```

### Input
```
yeet "what's ur name bestie?"
no_cap name be ratio
yeet "yo " + name
```

### Assert
```
caught_in_4k x > 0    # panics if false
```

### Comments
```
rent_free this is a comment
```

---

## Operator Reference

| Symbol / Keyword | Meaning        |
|------------------|----------------|
| `+`              | add / concat   |
| `-`              | subtract       |
| `*`              | multiply       |
| `/`              | divide         |
| `%`              | modulo         |
| `==`             | equals         |
| `!=`             | not equals     |
| `<` `>`          | comparison     |
| `<=` `>=`        | comparison     |
| `and`            | logical AND    |
| `or`             | logical OR     |
| `not`            | logical NOT    |

---

## Example Programs

| File                            | What it does                   |
|---------------------------------|-------------------------------|
| `examples/hello.bruh`           | Hello world                   |
| `examples/fizzbuzz.bruh`        | FizzBuzz 1–20                 |
| `examples/functions.bruh`       | Recursion: factorial, fib     |
| `examples/arrays.bruh`          | Arrays, ranges, push          |
| `examples/match.bruh`           | Pattern matching              |
| `examples/calculator.bruh`      | Calculator functions          |

---

## Testing

```bash
# Rust unit tests
cargo test

# .bruh integration tests
bash scripts/run_tests.sh
```

Tests live in `tests/bruh/`. Each test is a `.bruh` file paired with a `.expected` file containing the expected stdout.

---

## Contributing

PRs are welcome, no cap! See [CONTRIBUTING.md](CONTRIBUTING.md).

---

## License

Licensed under MIT. Chle and Contributors. All rights reserved.