# hyprdashboard
A dashboard on sterioid for Hyprland

## Icon Lookup

Icons are now located following the freedesktop.org Icon Theme Specification. The search uses directories from `$XDG_DATA_HOME` and each path listed in `$XDG_DATA_DIRS`. For the current theme (read from `$XDG_ICON_THEME` and falling back to `hicolor`) each `index.theme` file is parsed to discover subdirectories and inherited themes. Resolution specific directories are respected and fall back to `hicolor` if the icon is not found.


## Logging

The application uses `env_logger` for logging. Enable log output by setting
`RUST_LOG` before running the binary, e.g.:

```bash
RUST_LOG=info cargo run
```

btw: Try to learn some Rust with this tool and the help of AI :)
