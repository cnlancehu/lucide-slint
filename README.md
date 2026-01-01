<p align="center">
  <a href="https://github.com/cnlancehu/lucide-slint#gh-light-mode-only">
    <img src="./assets/logo-light.svg#gh-light-mode-only" width="600">
  </a>
  <a href="https://github.com/cnlancehu/lucide-slint#gh-dark-mode-only">
    <img src="./assets/logo-dark.svg#gh-dark-mode-only" width="600">
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

### Rust (Cargo)

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

### C++ (CMake)

For C++ projects using CMake, append the following to your `CMakeLists.txt` file to download and register the lucide-slint library:

```cmake
# Download lucide-slint library
set(LUCIDE_SLINT "${CMAKE_CURRENT_BINARY_DIR}/lucide.slint")
if(NOT EXISTS "${LUCIDE_SLINT}")
  file(DOWNLOAD "https://pkg.lance.fun/go/lucide-slint/latest/lib.slint" "${LUCIDE_SLINT}" SHOW_PROGRESS)
endif()

# Register the library path
slint_target_sources(my_application ui/app-window.slint
  LIBRARY_PATHS lucide=${LUCIDE_SLINT}
)
```

## Usage
Then you can use lucide icons in your Slint files like this:

<image align="right" src="./assets/example1.webp" width="20%" />

<!-- example 1 -->
```slint
import { IconDisplay, IconSet } from "@lucide";

export component Example {
    IconDisplay {
        icon: IconSet.Play; // set the icon to display
        stroke: #5E72E4;  // set the stroke color
        size: 48px;         // set the icon size
        stroke-width: 2;    // set the stroke width
    }
}
```


Or, you could just use icons with default `size`, `stroke` and `stroke-width`:

<image align="right" src="./assets/example2.webp" width="20%" />

<!-- example 2 -->
```slint
import { IconDisplay, IconSet } from "@lucide";

export component Example {
    IconDisplay {
        icon: IconSet.Columns3Cog;
    }
}
```

## Reference
### Icon Properties
These properties align with the standard Lucide icon configuration.

`IconDisplay` has the following properties:

| Property                | Type                                                                                 | Description                                                                    | Default       | Reference                                                                                   |
| ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------ | ------------- | ------------------------------------------------------------------------------------------- |
| `icon`                  | Icon                                                                                 | The specific icon to display from the IconSet enum                             | -             | -                                                                                           |
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

**Example:**

```slint
import { IconDisplay, IconSet } from "@lucide";

export component Example {
    IconDisplay {
        icon: IconSet.AArrowDown;
    }
}
```

## [**Try it in SlintPad**](https://snapshots.slint.dev/master/editor/?lib=lucide%3Dhttps%3A%2F%2Fpkg.lance.fun%2Fgo%2Flucide-slint%2Fnext%2Flib.slint&gz=H4sIAAAAAAAACpVPTWvDMAy9-1eIjMIGTfBua3Ipo5edC70bWwliju3JKsta8t9LUrqlsMOmg-A9vQ9EfYoscIYDspA1_jUOMELLsYciiys_yXUoucqeghSNopvhzcawo5y8-VrPYI_y7dz6oyWHRaMUDrPBxj7FgEFgh32EswKAu9IrM80iecFOQzaG-lZW7T-OxIy-udNkOmENz1qn4ecwqv-ni-E_JM8H4fiONQibkJNhDPKboGzJ-xq2ngIaLjs2jjDI40Y77Nbw0LbObBzoSuvVDF-s1lPfRDwtn7nuUV0Ae-ZJPb4BAAA%3D)

![screenshot](https://github.com/cnlancehu/lucide-slint/raw/main/assets/try-it-in-slintpad.webp)

Turn on the **View - Properties** panel and **select the icon** to modify icon properties with ease.

## License
This project is licensed under the MIT License, while Lucide is licensed under [the ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE).
