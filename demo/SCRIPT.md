# Demo script (60 seconds)

For recording the narrated version by hand. `demo.tape` is the same story without a voice,
rendered by the **Demo** workflow.

**Setup, off camera**

```sh
git clone https://github.com/keepjay-dev/keepjay-demo && cd keepjay-demo
export KEEPJAY_DATA_DIR=$(mktemp -d)   # a fresh, throwaway Keepjay
kj init -w Demo && cargo clean && cargo fetch && clear
```

Use a large font (about 20 pt) and a terminal about 110 columns wide.

| Time | On screen | Say |
|---|---|---|
| 0–8 s | `sed -n '32,40p' src/backoff.rs`: the `@card` comment above `delay` | "Keepjay turns comments in your code into flashcards. Here's one, right above the function it explains." |
| 8–15 s | `kj source add . && kj scan`: 4 cards found, ids written | "Register the repo and scan. It finds cards in comments in 30-odd languages, and in Markdown files." |
| 15–40 s | `kj while -- cargo test`: review two cards (space, then 3) while it builds; the code-block card is coloured | "Now the good part: review while your tests run. When they finish, you're back with their output and exit code." |
| 40–52 s | Edit the line under the card (in your editor, or `sed`), then `kj scan && kj stale` | "Cards are anchored to the code under them. Change that code, and Keepjay flags the card as stale instead of letting it quietly go wrong." |
| 52–60 s | `keepjay.com` | "Local-first, free, no account. Keepjay: keep what you learn while you code." |

**Tips**

- Do a dry run first: `cargo test` takes about 20–30 seconds from clean (the build, plus
  `tests/flaky_server.rs`, which really waits between retries for about 13 seconds).
- On macOS, the `sed` edit is `sed -i '' 's/pow(attempt)/pow(attempt + 1)/' src/backoff.rs`.
- Afterwards, `git checkout .` puts the files back (the scan writes card ids into them).
