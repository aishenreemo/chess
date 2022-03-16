## Chess
> a simple implementation of chess written in rust
---

> install [rust](https://www.rust-lang.org/tools/install) 
```
# Unix-like OS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

> compile the binary in your own system
```
git clone https://github.com/aishenreemo/chess
cd chess
cargo run
```

> contributing
```
# fork the repo then
git clone https://github.com/YOUR_USER_NAME/chess

cd chess

# make your feature branch
git branch my-branch
git checkout my-branch

# edit files and
git add .
git commit -m "feat: summary of your commit"

# running dev
cargo clippy --all --all-targets -- -D warnings
cargo fmt && cargo build && ./target/debug/chess
```