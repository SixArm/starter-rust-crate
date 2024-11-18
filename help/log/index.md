# Log

We previously used the crate `log` and implementation crate `env_logger`.

`Cargo.toml`

```toml
log = { version = "0.4.22" } # Lightweight logging facade.
env_logger = { version = "0.9.3" } # A logger that can be configured via environment variables.
```

`src/main.rs`

```rust
fn main() {
    env_logger::init();
    //…
}
```
