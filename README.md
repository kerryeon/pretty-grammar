# PrettyGrammar
[![Version](https://docs.rs/pretty_grammar/badge.svg)](https://crates.io/crates/pretty_grammar)
A library that supports simple grammar correction of dynamic strings.

# Installation
```toml
[dependencies]
pretty_grammar = "0.1"
```

# Usage
```rust
use pretty_grammar::translate;
use pretty_grammar::format_dyn;

fn main() {
    let source  = "{name}<은> {obj}<를> 좋아합니다.";
    let target = translate!(source with
        lang: "kr",  // Required
        name: "철수",
        obj: "영희"
    );
    println!("{}", target);
    // 철수는 영희를 좋아합니다.
}
```

# Supported Languages
| Target Language | Code | Supported |
|:---------------:|:----:|:---------:|
| Korean          | kr   | Yes       |
| English         | en   | Not Yet   |

# Documentation
[Docs.rs](https://docs.rs/pretty_grammar)

# License
Distributed under MIT License since 2019.
