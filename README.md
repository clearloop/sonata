## Cydonia

The static site generator in rust.

```toml
# my-blog/cydonia.toml
name = "Cydonia"          # The name of the site.

# Default values of the optional fields.
# --------------------------------------

favicon = "favicon.ico"   # The path to the favicon.ico.
posts = "posts"           # The path to the posts.
templates = "templates"   # The path to the templates.

# Theme could also be a folder:
#
# - [theme]
#   - index.css (required)
#   - post.css  (required)
#   - theme.css (optional)
theme = "theme.css"
```

## LICENSE

GPL-3.0-only
