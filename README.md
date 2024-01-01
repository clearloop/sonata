## Cydonia

The static site generator.

For the minimal directory layout:

```
.
├── cydonia.toml
└── posts
    └── 2024-01-01-hello-world.md
```

The full configuration:

```toml
# my-blog/cydonia.toml
title = "Cydonia"         # The title of the site.

# Default values of the optional fields.
# --------------------------------------
favicon = "favicon.svg"   # The path to the favicon.
out = "out"               # The path to the output directory.
posts = "posts"           # The path to the posts.
public = "public"         # The path to the public directory.
templates = "templates"   # The path to the templates.

# Theme could also be a folder:
#
# - [theme]
#   - index.css (optional)
#   - post.css  (optional)
#   - theme.css (optional)
theme = "theme.css"
```

## LICENSE

GPL-3.0-only
