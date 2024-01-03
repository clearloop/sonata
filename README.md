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

## Examples

| Example                                      | Description            |
| -------------------------------------------- | ---------------------- |
| [cydonia.toml][cydonia-toml]                 | The full configuration |
| [.github/workflows/cydonia.yml][cydonia-yml] | Github pages action    |

## LICENSE

GPL-3.0-only

[cydonia-toml]: blog/cydonia.toml
[cydonia-yml]: .github/workflows/cydonia.yml
[version-badge]: https://img.shields.io/crates/v/cydonia
[version-link]: https://docs.rs/cydonia
[ci-badge]: https://img.shields.io/github/actions/workflow/status/clearloop/cydonia/main.yml
[ci-link]: https://github.com/clearloop/cydonia/actions/workflows/main.yml
