# Live2d Parser

A Rust library for parsing Live2D model files, without any ffi calls.

Supports Cubism 2.0/3.0/4.0/4.0 formats.


## Examples

- Analyze Cubism 2.0 model

```rust
use live2d_parser::cubism_v1::Moc;

let moc = unsafe { Moc::new(include_bytes!("BCY.moc"))? };
let mut json = File::create("BCY.json")?;
json.write_all(serde_json::to_string_pretty(&moc)?.as_bytes())?;
```

- Analyze Cubism 3.0 model

```rust
use live2d_parser::cubism_v3::Moc3;

let moc = unsafe { Moc3::new(include_bytes!("mao_pro.moc3"))? };
let mut json = File::create("mao_pro.json")?;
json.write_all(serde_json::to_string_pretty(&moc)?.as_bytes())?;
```
