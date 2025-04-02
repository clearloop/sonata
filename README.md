# sonata

[![sonata][version-badge]][version-link]
[![ci][ci-badge]][ci-link]

## Usage

```bash
cargo install sonata
sonata init blog
sonata serve blog
```

The minimal directory layout is like below, see [sonata.toml](./blog/sonata.toml)
for the full configuration.

```
my-blog
├── sonata.toml
└── posts
    └── 2024-01-01-hello-world.md
```

## Github Action

```yaml
name: sonata

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
      - uses: clearloop/sonata@0.1.6

      - name: Build the site
        run: sonata build blog

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./blog/out
```

## LICENSE

GPL-3.0-only

[version-badge]: https://img.shields.io/crates/v/sonata
[version-link]: https://docs.rs/sonata
[ci-badge]: https://img.shields.io/github/actions/workflow/status/clearloop/sonata/main.yml
[ci-link]: https://github.com/clearloop/sonata/actions/workflows/main.yml
