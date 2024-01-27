## Lightweight Dependency Injection (DI) framework

### Crate
```toml
[dependencies]
fx-rs = { version = "1.0.0"}
```

### Usage
```rust
build_container! {
    #[derive(Clone)]
    pub struct ServiceProvider;
    
    attribute: <dyn Provider>
}
let providers = ServiceProvider::build().await?
let some_provider: NeedleProvider = providers.provide()

```