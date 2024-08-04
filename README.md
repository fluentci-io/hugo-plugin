# Hugo Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/hugo)](https://pkg.fluentci.io/hugo)
[![ci](https://github.com/fluentci-io/hugo-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/hugo-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [Hugo](https://gohugo.io/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm hugo build -s ./site -d ./public
```

## Functions

| Name   | Description                               |
| ------ | ----------------------------------------- |
| setup  | Install Hugo                              |
| build  | Build your site                           |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call(
  "https://pkg.fluentci.io/hugo@v0.1.0?wasm=1", 
  "build", 
  vec![
    "-s", "./site", 
    "-d", "./public"
  ])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: hugo
    args: |
      build -s ./site -d ./public
```
