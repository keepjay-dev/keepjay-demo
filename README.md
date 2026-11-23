# Keepjay demo

A tiny Rust crate with [Keepjay](https://github.com/keepjay-dev/keepjay) cards in it: try
Keepjay on it in two minutes, or record the demo from it.

## Try it

[Install Keepjay](https://github.com/keepjay-dev/keepjay#install), then:

```sh
git clone https://github.com/keepjay-dev/keepjay-demo && cd keepjay-demo
kj source add .              # register this repository
kj scan                      # finds 4 cards: 2 in comments, 2 in rust.cards.md
kj while -- cargo test       # review while the tests build and run
```

Then change the code under a card, say `pow(attempt)` in `src/backoff.rs`, and run
`kj scan && kj stale`: that card is flagged as stale.

The cards:

- `src/backoff.rs`: why backoff delays are random (full jitter).
- `src/lru.rs`: why an LRU cache's `get` takes `&mut self`.
- `rust.cards.md`: a code block that doesn't compile, and a cloze card about `Option<&T>`.

`tests/flaky_server.rs` retries against a simulated flaky server with real waits, so
`cargo test` takes long enough to review a couple of cards.

## The recorded demo

`demo/demo.tape` is the 60-second demo as a [VHS](https://github.com/charmbracelet/vhs)
script; the **Demo** workflow renders it to a GIF and an MP4 with Keepjay built from
`keepjay-dev/keepjay`. `demo/SCRIPT.md` is the narrated version, for recording by hand.

The code is MIT-licensed; copy it freely.
