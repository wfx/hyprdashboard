# hyprdashboard
A dashboard for Hyprland

## Configuration

`hyprdashboard` looks for `~/.config/hyprdashboard/config.toml` on start. The file is optional and can contain the following field:

```toml
# Theme name used when searching for application icons
icon_theme = "Adwaita"
```

If the file is missing or `icon_theme` is not provided, system icons will be searched without a specific theme.
