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

# Lucide Slint
Implementation of the [lucide icon library](https://github.com/lucide-icons/lucide) for Slint.

# Installation
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
        PathBuf::from(lucide_slint::get_slint_file_path().to_string()),
    )]);
    let config = slint_build::CompilerConfiguration::new().with_library_paths(library);

    // Specify your Slint code entry here
    slint_build::compile_with_config("ui/main.slint", config).expect("Slint build failed");
}
```

# Usage
Then you can use lucide icons in your Slint files like this:

```slint
import { PlayIcon } from "@lucide";

export component App inherits Window {
    VerticalBox {
        PlayIcon {
            size: 24px;
            colorize: #fff;
        }
    }
}
```

The Icon component inherits an [`Image element`](https://docs.slint.dev/latest/docs/slint/reference/elements/image/), so you can set properties like `vertical-tiling`, `width`, etc.

```slint
import { FlowerIcon } from "@lucide";

FlowerIcon {
    size: 36px;
    width: 100%;
    height: 100%;
    opacity: 0.7;
    vertical-tiling: round;
    horizontal-tiling: round;
}
```

Or if you just want to use the raw [image](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/#image) data, you can access the icon via `Icons`:

```slint
import { Icons } from "@lucide";

Button {
    text: "Back";
    icon: Icons.ArrowLeftIcon;
    colorize-icon: true;
}

// equivalent to
Button {
    text: "Back";
    icon: @image-url("icons/arrow-left.svg"); // The path is actually relative to the lucide-slint package, use `Icons.ArrowLeftIcon` instead.
    colorize-icon: true;
}
```

# Available Icons

For a complete list of available icons, visit the [Lucide Icons](https://lucide.dev/icons/) website.

To use an icon in Slint:
1. Find your desired icon (e.g., `a-arrow-down`)
2. Click **Copy Component Name** to get the PascalCase name (e.g., `AArrowDown`)
![Copy Component Name](./assets/copy-component-name.png)
3. Append `Icon` to the component name: `AArrowDownIcon`

**Example:**

```slint
import { AArrowDownIcon } from "@lucide";

AArrowDownIcon { }
```

# License
This project is licensed under the MIT License, while Lucide is licensed under the ISC License.

See [LICENSE](./LICENSE) for more details.
