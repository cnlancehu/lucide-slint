//! ## Lucide Slint
//!
//! Implementation of the [Lucide icon library](https://github.com/lucide-icons/lucide)
//! for [Slint](https://github.com/slint-ui/slint).
//!
//! Use lucide icons in your Slint applications with ease!
//!
//! ## [Documentation](https://github.com/cnlancehu/lucide-slint)
//!
//! Please refer to [README](https://github.com/cnlancehu/lucide-slint)

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
