# GPUIX

A community fork of [GPUI](https://gpui.rs), Zed's GPU-accelerated UI framework.

## Usage

### Use `gpui` and `gpui_platform`

```toml
[dependencies]
gpui = { git = "https://github.com/AzureZee/gpuix.git" }
gpui_platform = { git = "https://github.com/AzureZee/gpuix.git" }
```

```rs
use gpui::{import};
use gpui_platform::application;

fn main() {
  application().run(|cx: &mut App| {/*...*/});
}
```

### Or use `gpuix`

```toml
# you can do this
[dependencies]
gpui = { git = "https://github.com/AzureZee/gpuix.git", package = "gpuix" }

# or this
[dependencies]
gpuix = { git = "https://github.com/AzureZee/gpuix.git" }
```

```rs
use gpui::{import,application};
// or this
// use gpuix::{import,application};

fn main() {
  application().run(|cx: &mut App| {/*...*/});
}
```
