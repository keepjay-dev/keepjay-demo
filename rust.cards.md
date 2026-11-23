---
tags: [lang::rust]
---

# Rust cards

Cards that don't belong next to any one piece of code.

Q: Why doesn't this compile?
```rust
let s = String::from("hi");
let t = s;
println!("{s}");
```
A: `String` isn't `Copy`, so `let t = s` **moves** it and `s` can't be used afterwards. Clone it (`s.clone()`) or borrow it (`let t = &s`).

C: `Option<&T>` is the same size as `&T`, thanks to the {{c1::niche optimisation}}: a null pointer stands for `None`.
