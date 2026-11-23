# Keepjay's 60-second demo, as a VHS tape (https://github.com/charmbracelet/vhs).
# Render it with the Demo workflow, or locally from a fresh clone: `vhs demo/demo.tape`.
# The story: cards live in comments, `while` reviews during a build, and a card goes
# stale when the code under it changes. See demo/SCRIPT.md for the narrated version.

Output demo/out/demo.gif
Output demo/out/demo.mp4

Require kj
Require cargo

Set Shell "bash"
Set FontSize 20
Set Width 1400
Set Height 660
Set Padding 24
Set TypingSpeed 40ms
Set Theme "Catppuccin Mocha"

# Off camera: a throwaway data folder, no usage ping, a clean build, and a plain prompt.
# `cargo test` then takes about 20 seconds: the build, and tests/flaky_server.rs, which
# really waits between retries.
Hide
Type "export KEEPJAY_DATA_DIR=$(mktemp -d) KEEPJAY_NO_TELEMETRY=1 PS1='\[\e[1;34m\]demo\[\e[0m\] $ '" Enter
Type "kj init -w Demo > /dev/null && cargo clean -q && cargo fetch -q && clear" Enter
Sleep 3s
Show

# 1. A card is a comment above the code it explains.
Type "# Cards live in your code, next to what they explain:" Enter
Sleep 500ms
Type "sed -n '32,40p' src/backoff.rs" Enter
Sleep 4s
Screenshot demo/out/1-card.png

# 2. Register the repository and scan it.
Type "kj source add . && kj scan" Enter
Sleep 3s
Screenshot demo/out/2-scan.png

# 3. Review while the tests build and run.
Type "clear" Enter
Type "kj while -- cargo test" Enter
Wait+Screen@30s /space reveal/
Sleep 3s
Space
Sleep 4s
Screenshot demo/out/3-review.png
Type "3"
Wait+Screen@10s /space reveal/
Sleep 3s
Space
Sleep 3.5s
Screenshot demo/out/4-review.png
Type "3"
Wait+Screen@90s /test result: ok/
Sleep 3s
Screenshot demo/out/5-done.png

# 4. Change the code under a card: Keepjay notices.
Type "clear" Enter
Type "# Later, the code under a card changes..." Enter
Sleep 500ms
Type "sed -i 's/pow(attempt)/pow(attempt + 1)/' src/backoff.rs" Enter
Sleep 1s
Type "kj scan && kj stale" Enter
Sleep 5s
Screenshot demo/out/6-stale.png

Type "# keepjay.com" Enter
Sleep 3s
