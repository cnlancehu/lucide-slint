<p align="center">
  <a href="https://github.com/cnlancehu/lucide-slint">
    <img src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/logo-dark.svg" width="600">
  </a>
</p>

<p align="center">
  <a href="https://crates.io/crates/lucide-slint"><img alt="Crates.io" src="https://img.shields.io/crates/v/lucide-slint"></a>
</p>

<p align="center">
  <a href="https://crates.io/crates/lucide-slint">crates.io</a>
  ·
  <a href="https://docs.rs/lucide-slint/">Documentation</a>
</p>

## Lucide Slint
Implementation of the [lucide icon library](https://github.com/lucide-icons/lucide) for [Slint](https://github.com/slint-ui/slint).

Use lucide icons in your Slint applications with ease!

## Features

**🚀 Optimized Performance**

All icons are pre-converted to [Path element](https://docs.slint.dev/latest/docs/slint/reference/elements/path/), eliminating runtime SVG parsing overhead for better performance and reduced memory footprint.

**🎨 Full Property Support**

All configuration properties from the official Lucide package are supported, giving you complete control over icon appearance.

## ⚠️ Notice

**Lucide Slint 0.2.0 and later** requires **Slint 1.15+** (will be released in the future) or the [master branch](https://github.com/slint-ui/slint/tree/master).

For **Slint 1.14.x**, please use **Lucide Slint [0.1.4](https://crates.io/crates/lucide-slint/0.1.4)** and refer to its [documentation](https://docs.rs/lucide-slint/0.1.4/lucide_slint/).

**Lucide Slint 0.2.0** depends on features introduced in [this PR](https://github.com/slint-ui/slint/pull/9912), which is not yet released in the stable version.

## Installation
In an existing Slint project, run the following command to add lucide-slint as a **build** dependency:

```bash
cargo add lucide-slint --build
```

Add the following to your `build.rs` file to import `lucide-slint` as a Slint library:

```rust
use std::{collections::HashMap, path::PathBuf};

fn main() {
    let library = HashMap::from([(
        "lucide".to_string(),
        PathBuf::from(lucide_slint::lib()),
    )]);
    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(library);

    // Specify your Slint code entry here
    slint_build::compile_with_config("ui/main.slint", config)
        .expect("Slint build failed");
}
```

## Usage
Then you can use lucide icons in your Slint files like this:

<image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example1.webp" width="20%" />

<!-- example 1 -->
```slint
import { PlayIcon } from "@lucide";

export component App inherits Window {
    PlayIcon {
        stroke: #5E72E4;    // set the stroke color
        size: 48px;         // set the icon size
        stroke-width: 2;    // set the stroke width
    }
}
```


Or, you could just use icons with default `size`, `stroke` and `stroke-width`:

<image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example2.webp" width="20%" />

<!-- example 2 -->
```slint
import { Columns3CogIcon } from "@lucide";

Columns3CogIcon { }
```

Modify the default values:

<image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example3.webp" width="20%" />

```slint
import { PlayIcon, IconSettings } from "@lucide";

init => {
    IconSettings.default-stroke = #2dce89;
    IconSettings.default-size = 48px;
    IconSettings.default-stroke-width = 1.0;
    IconSettings.default-stroke-fill = transparent;
    IconSettings.default-absolute-stroke-width = false;
}
PlayIcon { }
```

## Reference
### Icon Properties
These properties align with the standard Lucide icon configuration.

All icons have the following properties:

| Property                | Type                                                                                 | Description                                                                    | Default       | Reference                                                                                   |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------ | ------------- | ------------------------------------------------------------------------------------------- |
| `size`                  | [length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/#length) | The size of the icon                                                           | `24px`        | [Sizing](https://lucide.dev/guide/basics/sizing)                                            |
| `stroke`                | [brush](https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/#_top) | The stroke color of the icon                                                   | `white`       | [Color](https://lucide.dev/guide/basics/color)                                              |
| `stroke-fill`           | [brush](https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/#_top) | The stroke fill color of the icon                                              | `transparent` | [Filled Icons](https://lucide.dev/guide/advanced/filled-icons)                              |
| `stroke-width`          | float  (unit: px)                                                                    | The stroke width of the icon                                                   | `2`           | [Stroke width](https://lucide.dev/guide/basics/stroke-width#stroke-width)                   |
| `absolute-stroke-width` | bool                                                                                 | Whether the size of the stroke width will be relative to the size of the icon. | `false`       | [Absolute stroke width](https://lucide.dev/guide/basics/stroke-width#absolute-stroke-width) |

### Icon Out properties
All icons have the following out properties:

| Property                  | Type                                                                                 | Description                                                                                                               |
| ------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
| `calculated-stroke-width` | [length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/#length) | The real stroke width of the icon calculated according to the `stroke-width`, `size` and `absolute-stroke-width` property |

## Available Icons

For a complete list of available icons, visit the [Lucide Icons](https://lucide.dev/icons/) website.

To use an icon in Slint:
1. Find your desired icon: `a-arrow-down`
2. Click **Copy Component Name** to get the PascalCase name: `AArrowDown`
   ![Copy Component Name](./assets/copy-component-name.png)
3. Append `Icon` to the component name: `AArrowDownIcon`

**Example:**

```slint
import { AArrowDownIcon } from "@lucide";

AArrowDownIcon { }
```

## [**Try it in SlintPad**](https://snapshots.slint.dev/master/editor/?lib=lucide%3Dhttps%3A%2F%2Fpkg.lance.fun%2Fgo%2Flucide-slint%2Fnext%2Flib.slint&gz=H4sIAAAAAAAACnVQwWrDMAy95ytEx2CDJvV2WhM6ythl58DuXiynYo6d2QoLK_n34bQrScl0MPLT89Pzo6Z1nuEI7-iZKmleXA8DaO8aWAVW6TepGjlkwZDlVZHQ34PyqyPv0bxVzq6hZOlPXTxLZCZbh4vS3nQVKVwVSYL9KFC5pnUWLcMrNg6OCQDMTJyQWGSJYfc8QWJN92QKtewMp4F-EHbwIETbF7DZQIkMfEA4E4AqZ2FksTvRLqLDpZt-7WprYO8-MYcbrXWxMIl58SGHx_lQfgRnOsZ0ztLSBCyWHJzj_Gc7e2lDKz1aXjShyZgc9oYsSp_WXipCy3dbobBeR-9KbhWITIjb8fpUCRHTiMD9tZ8hGZJfUcN6DycCAAA%3D)

![screenshot](https://github.com/cnlancehu/lucide-slint/raw/main/assets/try-it-in-slintpad.webp)

Turn on the **View - Properties** panel and **select the icon** to modify icon properties with ease.
