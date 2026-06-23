# GitHub Actions CI

use this workflow in `.github/workflows/multi-platform.yml` to build the mod. never publish locally built geode files.

```yaml
name: Build Geode Mod

on:
  push:
    branches:
      - main
  pull_request:

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        config:
        - name: Windows
          os: windows-latest
        - name: macOS
          os: macos-latest
        - name: Android
          os: ubuntu-latest
          target: Android

    name: ${{ matrix.config.name }}
    runs-on: ${{ matrix.config.os }}

    steps:
    - uses: actions/checkout@v4

    - name: Setup Rust
      uses: dtolnay/rust-toolchain@stable

    - name: Build Mod
      uses: geode-sdk/build-geode-mod@main
      with:
        sdk: false
        target: ${{ matrix.config.target }}
```

### example

#### cpp side and rust side

cpp side and rust side are not needed for github actions ci workflow config just configure the repository workflow file as shown above

