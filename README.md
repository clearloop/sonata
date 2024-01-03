# Cydonia

[![cydonia][version-badge]][version-link]
[![ci][ci-badge]][ci-link]

## Usage

```bash
cargo install cydonia
cydonia init blog
cydonia serve blog
```

The minimal directory layout is like below, see [cydonia.toml](./blog/cydonia.toml)
for the full configuration.

```
my-blog
├── cydonia.toml
└── posts
    └── 2024-01-01-hello-world.md
```

## Github Action

```yaml
name: Cydonia

on:
  push:
    branches: [main]

jobs:
  deploy:
    name: Deploy
    runs-on: ubuntu-22.04
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4
      - uses: clearloop/cydonia@0.0.7

      - name: Build the site
        run: cydonia build blog

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./blog/out
```

## LICENSE

GPL-3.0-only

[version-badge]: https://img.shields.io/crates/v/cydonia
[version-link]: https://docs.rs/cydonia
[ci-badge]: https://img.shields.io/github/actions/workflow/status/clearloop/cydonia/main.yml
[ci-link]: https://github.com/clearloop/cydonia/actions/workflows/main.yml
