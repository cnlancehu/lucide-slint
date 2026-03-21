//! <p align="center">
//!   <a href="https://github.com/cnlancehu/lucide-slint#gh-light-mode-only">
//!     <img src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/logo-light.svg#gh-light-mode-only" width="600">
//!   </a>
//!   <a href="https://github.com/cnlancehu/lucide-slint#gh-dark-mode-only">
//!     <img src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/logo-dark.svg#gh-dark-mode-only" width="600">
//!   </a>
//! </p>
//!
//! <p align="center">
//!   <a href="https://crates.io/crates/lucide-slint"><img alt="Crates.io" src="https://img.shields.io/crates/v/lucide-slint"></a>
//! </p>
//!
//! <p align="center">
//!   <a href="https://crates.io/crates/lucide-slint">crates.io</a>
//!   ·
//!   <a href="https://docs.rs/lucide-slint/">Documentation</a>
//! </p>
//!
//! ## Lucide Slint
//!
//! Implementation of the [lucide icon library](https://github.com/lucide-icons/lucide)
//! for [Slint](https://github.com/slint-ui/slint).
//!
//! Use lucide icons in your Slint applications with ease!
//!
//! ## Features
//!
//! **🚀 Optimized Performance**
//!
//! All icons are pre-converted to [Path element](https://docs.slint.dev/latest/docs/slint/reference/elements/path/),
//! eliminating runtime SVG parsing overhead for better performance and reduced memory footprint.
//!
//! **🎨 Full Property Support**
//!
//! All configuration properties from the official Lucide package are supported, giving you complete control over icon
//! appearance.
//!
//! ## Installation
//!
//! ### Requirements
//!
//! The latest version of lucide-slint requires **Slint 1.15+**.
//!
//! Please ensure your project is using Slint 1.15 or later to avoid compatibility issues.
//!
//! ### Rust (Cargo)
//!
//! In an existing Slint project, run the following command to add lucide-slint as a **build** dependency:
//!
//! ```bash
//! cargo add lucide-slint --build
//! ```
//!
//! Add the following to your `build.rs` file to import `lucide-slint` as a Slint library:
//!
//! ```rust
//! use std::{collections::HashMap, path::PathBuf};
//!
//! fn main() {
//!     let library = HashMap::from([(
//!         "lucide".to_string(),
//!         PathBuf::from(lucide_slint::lib()),
//!     )]);
//!     let config = slint_build::CompilerConfiguration::new()
//!         .with_library_paths(library);
//!
//!     // Specify your Slint code entry here
//!     slint_build::compile_with_config("ui/main.slint", config)
//!         .expect("Slint build failed");
//! }
//! ```
//!
//! ### C++ (CMake)
//!
//! For C++ projects using CMake, append the following to your `CMakeLists.txt` file to download and register the
//! lucide-slint library:
//!
//! ```cmake
//! # Download lucide-slint library
//! set(LUCIDE_SLINT "${CMAKE_CURRENT_BINARY_DIR}/lucide.slint")
//! if(NOT EXISTS "${LUCIDE_SLINT}")
//!   file(DOWNLOAD "https://github.com/cnlancehu/lucide-slint/releases/latest/download/lib.slint" "${LUCIDE_SLINT}" SHOW_PROGRESS)
//! endif()
//!
//! # Specify your Slint code entry here
//! slint_target_sources(my_application ui/main.slint
//!   LIBRARY_PATHS lucide=${LUCIDE_SLINT}
//! )
//! ```
//!
//! ### Manual
//!
//! Download the latest `lib.slint` from the [releases page](https://github.com/cnlancehu/lucide-slint/releases/latest) and place it in your project.
//!
//! ## Usage
//!
//! Then you can use lucide icons in your Slint files like this:
//!
//! <image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example1.webp" width="20%" />
//!
//! ```slint
//! import { IconDisplay, IconSet } from "@lucide";
//!
//! export component Example {
//!     IconDisplay {
//!         icon: IconSet.FilePlay;   // set the icon to display
//!         stroke: #c8f3e5;        // set the stroke color
//!         stroke-width: 1.5;        // set the stroke width
//!         size: 24px;               // set the icon size
//!     }
//! }
//! ```
//!
//! Or, you could just use icons with default `size`, `stroke` and `stroke-width`:
//!
//! <image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example2.webp" width="20%" />
//!
//! ```slint
//! import { IconDisplay, IconSet } from "@lucide";
//!
//! export component Example {
//!     IconDisplay {
//!         icon: IconSet.ScanText;
//!     }
//! }
//! ```
//!
//! Try `stroke-fill` property to get filled icons:
//!
//! **Note:** The `stroke-fill` property only works for icons that have a defined fill area. Also, it is not guaranteed that all icons will look good when filled.
//!
//! <image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example3.webp" width="20%" />
//!
//! ```slint
//! import { IconDisplay, IconSet } from "@lucide";
//!
//! export component Example {
//!     VerticalLayout {
//!         IconDisplay {
//!             icon: IconSet.TreeDeciduous;
//!             stroke: #7faf6a;
//!         }
//!
//!         IconDisplay {
//!             icon: IconSet.TreeDeciduous;
//!             stroke: #7faf6a;
//!             stroke-fill: #a1c88f;
//!         }
//!     }
//! }
//! ```
//!
//! Customize the icon display by inheriting `IconDisplay` to reuse your desired properties:
//!
//! <image align="right" src="https://github.com/cnlancehu/lucide-slint/raw/main/assets/example4.webp" width="20%" />
//!
//! ```slint
//! import { IconDisplay, IconSet } from "@lucide";
//!
//! // My custom icon display component with purple stroke and a stroke width of 1.5
//! export component MyIconDisplay inherits IconDisplay {
//!     stroke: #8e8cd8;
//!     stroke-width: 1.5;
//! }
//!
//! export component Example {
//!     VerticalLayout {
//!         MyIconDisplay {
//!             icon: IconSet.NotebookPen;
//!         }
//!
//!         MyIconDisplay {
//!             icon: IconSet.LampDesk;
//!         }
//!     }
//! }
//! ```
//!
//! ## Reference
//!
//! ### Icon Properties
//!
//! These properties align with the standard Lucide icon configuration.
//!
//! `IconDisplay` has the following properties:
//!
//! | Property                | Type                                                                                 | Description                                                                    | Default       | Reference                                                                                   |
//! | ----------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------ | ------------- | ------------------------------------------------------------------------------------------- |
//! | `icon`                  | Icon                                                                                 | The specific icon to display from the IconSet enum                             | -             | -                                                                                           |
//! | `size`                  | [length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/#length) | The size of the icon                                                           | `24px`        | [Sizing](https://lucide.dev/guide/basics/sizing)                                            |
//! | `stroke`                | [brush](https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/#_top) | The stroke color of the icon                                                   | `white`       | [Color](https://lucide.dev/guide/basics/color)                                              |
//! | `stroke-fill`           | [brush](https://docs.slint.dev/latest/docs/slint/reference/colors-and-brushes/#_top) | The stroke fill color of the icon                                              | `transparent` | [Filled Icons](https://lucide.dev/guide/advanced/filled-icons)                              |
//! | `stroke-width`          | float  (unit: px)                                                                    | The stroke width of the icon                                                   | `2`           | [Stroke width](https://lucide.dev/guide/basics/stroke-width#stroke-width)                   |
//! | `absolute-stroke-width` | bool                                                                                 | Whether the size of the stroke width will be relative to the size of the icon. | `false`       | [Absolute stroke width](https://lucide.dev/guide/basics/stroke-width#absolute-stroke-width) |
//!
//! ### Icon Out properties
//!
//! `IconDisplay` have the following out properties:
//!
//! | Property                  | Type                                                                                 | Description                                                                                                               |
//! | ------------------------- | ------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------- |
//! | `calculated-stroke-width` | [length](https://docs.slint.dev/latest/docs/slint/reference/primitive-types/#length) | The real stroke width of the icon calculated according to the `stroke-width`, `size` and `absolute-stroke-width` property |
//!
//! ## Available Icons
//!
//! For a complete list of available icons, visit the [Lucide Icons](https://lucide.dev/icons/) website.
//!
//! To use an icon in Slint:
//!
//! 1. Find your desired icon: `a-arrow-down`
//! 2. Click **Copy Component Name** to get the PascalCase name: `AArrowDown`
//!    ![Copy Component Name](https://github.com/cnlancehu/lucide-slint/raw/main/assets/copy-component-name.png)
//!
//! **Example:**
//!
//! ```slint
//! import { IconDisplay, IconSet } from "@lucide";
//!
//! export component Example {
//!     IconDisplay {
//!         icon: IconSet.AArrowDown;
//!     }
//! }
//! ```
//!
//! ## [**Try it in SlintPad**](https://slintpad.com/?lib=lucide=https://pkg.lance.fun/go/lucide-slint/latest/lib.slint&gz=H4sIAAAAAAAACpVPTWvDMAy9-1eIjMIGTfBua3Ipo5edC70bWwliju3JKsta8t9LUrqlsMOmg-A9vQ9EfYoscIYDspA1_jUOMELLsYciiys_yXUoucqeghSNopvhzcawo5y8-VrPYI_y7dz6oyWHRaMUDrPBxj7FgEFgh32EswKAu9IrM80iecFOQzaG-lZW7T-OxIy-udNkOmENz1qn4ecwqv-ni-E_JM8H4fiONQibkJNhDPKboGzJ-xq2ngIaLjs2jjDI40Y77Nbw0LbObBzoSuvVDF-s1lPfRDwtn7nuUV0Ae-ZJPb4BAAA=)
//!
//! ![screenshot](https://github.com/cnlancehu/lucide-slint/raw/main/assets/try-it-in-slintpad.webp)
//!
//! Turn on the **View - Properties** panel and **select the icon** to modify icon properties with ease.
//!
//! ## License
//!
//! This project is licensed under the MIT License, while Lucide is licensed under [the ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE).

/// Returns the file path to the `lib.slint` file included in this crate.
///
/// ## Example in `build.rs`
///
/// ```rust
/// use std::{collections::HashMap, path::PathBuf};
///
/// fn main() {
///     let library = HashMap::from([(
///         "lucide".to_string(),
///         PathBuf::from(lucide_slint::lib()),
///     )]);
///     let config = slint_build::CompilerConfiguration::new()
///         .with_library_paths(library);
///
///     // Specify your Slint code entry here
///     slint_build::compile_with_config("ui/main.slint", config)
///         .expect("Slint build failed");
/// }
/// ```
pub fn lib() -> String {
    concat!(env!("CARGO_MANIFEST_DIR"), "/lib.slint").to_string()
}
