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
    
    {some_name}: <dyn Provider>
    ....
}
let providers = ServiceProvider::build().await?
let some_provider: {some_provider} = providers.provide()

```