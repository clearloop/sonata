## Cydonia

[![cydonia][version-badge]][version-link]
[![ci][ci-badge]][ci-link]

```bash
cargo install cydonia
cydonia init blog
cydonia serve blog
```

For the minimal directory layout:

```
my-blog
├── cydonia.toml
└── posts
    └── 2024-01-01-hello-world.md
```

see [blog/cydonia.toml][cydonia-toml] for the full configuration of
`cydonia.toml`, [.github/workflows/action.yml][action-yml] for configuring
github actions .

## LICENSE

GPL-3.0-only

[action-yml]: .github/workflows/action.yml
[cydonia-toml]: blog/cydonia.toml
[version-badge]: https://img.shields.io/crates/v/cydonia
[version-link]: https://docs.rs/cydonia
[ci-badge]: https://img.shields.io/github/actions/workflow/status/clearloop/cydonia/main.yml
[ci-link]: https://github.com/clearloop/cydonia/actions/workflows/main.yml
