//! Tauri Specta will generate a [Typescript](https://www.typescriptlang.org) or [JSDoc](https://jsdoc.app) file (powered by [Specta](https://docs.rs/specta)) to provide a typesafe interface to your Tauri backend.
//!
//! ## Installation
//!
//! <section class="warning">
//!
//! Tauri Specta v2 is still in beta, and requires using [Tauri v2](https://tauri.app) and [Specta v2](https://github.com/specta-rs/specta) lands as stable.
//!
//! It is really important you use `=` in your versions to ensure your project will not break after future updates!
//!
//! </section>
//!
//! To get started run the following commands to add the required dependencies to your `Cargo.toml`:
//!
//! ```sh
//! # Always required
//! cargo add tauri@2.0 specta@=2.0.0-rc.21
//!
//! # Typescript
//! cargo add specta-typescript@0.0.9
//! cargo add tauri-specta@=2.0.0-rc.21 --features derive,typescript
//!
//! # JSDoc
//! cargo add specta-jsdoc@0.0.9
//! cargo add tauri-specta@=2.0.0-rc.21 --features derive,javascript
//! ```
//!
//! ## Features
//!
//! There are the following optional features which can be enabled:
//!
//! - `derive` - Enables the `Event` derive macro. This is only required if your using events.
//! - `javascript` - Enables the JSDoc exporter.
//! - `typescript` - Enables the Typescript exporter.
//!
//! ## Setup
//!
//! The follow is a minimal example of how to setup Tauri Specta with Typescript.
//!
//! ```rust,ignore
//! #![cfg_attr(
//!     all(not(debug_assertions), target_os = "windows"),
//!     windows_subsystem = "windows"
//! )]
//!
//! use serde::{Deserialize, Serialize};
//! use specta_typescript::Typescript;
//! use souchy_tauri_specta::{collect_commands, Builder};
//!
//! #[tauri::command]
//! #[specta::specta] // < You must annotate your commands
//! fn hello_world(my_name: String) -> String {
//!     format!("Hello, {my_name}! You've been greeted from Rust!")
//! }
//!
//! fn main() {
//!     let mut builder = Builder::<tauri::Wry>::new()
//!         // Then register them (separated by a comma)
//!         .commands(collect_commands![hello_world,]);
//!
//!     #[cfg(debug_assertions)] // <- Only export on non-release builds
//!     builder
//!         .export(Typescript::default(), "../src/bindings.ts")
//!         .expect("Failed to export typescript bindings");
//!
//!     tauri::Builder::default()
//!         // and finally tell Tauri how to invoke them
//!         .invoke_handler(builder.invoke_handler())
//!         .setup(move |app| {
//!             // This is also required if you want to use events
//!             builder.mount_events(app);
//!
//!             Ok(())
//!         })
//!         // on an actual app, remove the string argument
//!         .run(tauri::generate_context!("tests/tauri.conf.json"))
//!         .expect("error while running tauri application");
//! }
//! ```
//!
//! ## Export to JSDoc
//!
//! If your interested in using JSDoc instead of Typescript you can replace the [`specta_typescript::Typescript`](https://docs.rs/specta-typescript/latest/specta_typescript/struct.Typescript.html) struct
//! with [`specta_jsdoc::JSDoc`](https://docs.rs/specta-jsdoc/latest/specta_jsdoc/struct.JSDoc.html) like the following:
//!
//! ```rust,ignore-windows
//! let mut builder = souchy_tauri_specta::Builder::<tauri::Wry>::new();
//!
//! #[cfg(debug_assertions)]
//! builder
//!     .export(specta_jsdoc::JSDoc::default(), "../src/bindings.js")
//!     .expect("Failed to export typescript bindings");
//! ```
//!
//! ## Usage on frontend
//!
//! ```typescript
//! import { commands, events } from "./bindings"; // This should point to the file we export from Rust
//!
//! console.log(await commands.greet("Brendan"));
//! ```
//!
//! ## Custom types
//!
//! Similar to [`serde::Serialize`] you must put the [`specta::Type`] derive macro on your own types to allow Specta to understand your types. For example:
//! ```rust,ignore-windows
//! use serde::{Serialize, Deserialize};
//! use specta::Type;
//!
//! #[derive(Serialize, Deserialize, Type)]
//! pub struct MyStruct {
//!     a: String
//! }
//!
//! // Call `typ()` as much as you want.
//! let mut builder = souchy_tauri_specta::Builder::<tauri::Wry>::new().typ::<MyStruct>();
//! ```
//!
//! ## Events
//!
//! You can also make events typesafe by following the following example:
//!
//! ```rust,ignore-windows
//! use serde::{Serialize, Deserialize};
//! use specta::Type;
//! use souchy_tauri_specta::{Builder, collect_commands, collect_events, Event};
//!
//! // Add `souchy_tauri_specta::Event` to your event
//! #[derive(Serialize, Deserialize, Debug, Clone, Type, Event)]
//! pub struct DemoEvent(String);
//!
//! let mut builder = Builder::<tauri::Wry>::new()
//!         // and then register it to your builder
//!         .events(collect_events![DemoEvent]);
//!
//! tauri::Builder::default()
//!         .invoke_handler(builder.invoke_handler())
//!         .setup(move |app| {
//!             // Ensure you mount your events!
//!             builder.mount_events(app);
//!
//!             // Now you can use them
//!
//!             DemoEvent::listen(app, |event| {
//!                 println!("{:?}", event.payload);
//!             });
//!
//!             DemoEvent("Test".into()).emit(app).unwrap();
//!
//!             Ok(())
//!         });
//! ```
//!
//! and it can be used on the frontend like the following:
//!
//! ```ts
//! import { commands, events } from "./bindings";
//! import { appWindow } from "@tauri-apps/api/window";
//!
//! // For all windows
//! events.demoEvent.listen((e) => console.log(e));
//!
//! // For a single window
//! events.demoEvent(appWindow).listen((e) => console.log(e));
//!
//! // Emit to the backend and all windows
//! await events.demoEvent.emit("Test")
//!
//! // Emit to a window
//! await events.demoEvent(appWindow).emit("Test")
//! ```
//!
//! Refer to [`Event`] for all the possible methods for listening and emitting events.
//!
//! # Channel
//!
//! [Coming soon...](https://github.com/specta-rs/tauri-specta/issues/111)
//!
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc(
    // TODO: Tauri Specta logo
    html_logo_url = "https://github.com/specta-rs/specta/raw/main/.github/logo-128.png",
    html_favicon_url = "https://github.com/specta-rs/specta/raw/main/.github/logo-128.png"
)]

use core::fmt;
use std::{borrow::Cow, collections::BTreeMap, path::Path, sync::Arc};

use specta::{
    datatype::{self, DataType},
    SpectaID, TypeMap,
};

use tauri::{ipc::Invoke, Runtime};
/// Implements the [`Event`](trait@crate::Event) trait for a struct.
///
/// Refer to the [`Event`](trait@crate::Event) trait for more information.
///
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use souchy_tauri_specta_macros::Event;

mod builder;
mod event;
mod lang;
mod macros;

pub use builder::Builder;
pub(crate) use event::EventRegistry;
pub use event::{Event, TypedEvent};

/// A wrapper around the output of the `collect_commands` macro.
///
/// This acts to seal the implementation details of the macro.
#[derive(Clone)]
pub struct Commands<R: Runtime>(
    // Bounds copied from `tauri::Builder::invoke_handler`
    pub(crate) Arc<dyn Fn(Invoke<R>) -> bool + Send + Sync + 'static>,
    pub(crate) fn(&mut TypeMap) -> Vec<datatype::Function>,
    /// Module path for each command
    pub(crate) Vec<&'static str>,
);

impl<R: Runtime> fmt::Debug for Commands<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Commands").finish()
    }
}

impl<R: Runtime> Default for Commands<R> {
    fn default() -> Self {
        Self(
            Arc::new(tauri::generate_handler![]),
            ::specta::function::collect_functions![],
            vec![],
        )
    }
}

/// A wrapper around the output of the `collect_commands` macro.
///
/// This acts to seal the implementation details of the macro.
#[derive(Default)]
pub struct Events(BTreeMap<&'static str, fn(&mut TypeMap) -> (SpectaID, DataType)>);

/// The context of what needs to be exported. Used when implementing [`LanguageExt`].
#[derive(Debug, Clone)]
#[non_exhaustive]
#[allow(missing_docs)]
pub struct ExportContext {
    pub plugin_name: Option<&'static str>,
    pub commands: Vec<datatype::Function>,
    pub command_modules: Vec<&'static str>,
    pub error_handling: ErrorHandlingMode,
    pub events: BTreeMap<&'static str, DataType>,
    pub type_map: TypeMap,
    pub constants: BTreeMap<Cow<'static, str>, serde_json::Value>,
    pub per_file: bool,
}

/// Exports files content
#[derive(Default, Debug, Clone)]
pub struct ExportFiles {
    /// File content keyed by file name
    pub content_per_file: BTreeMap<String, String>,
}
impl ExportFiles {
    /// Creates a new instance of `ExportFiles`.
    pub fn new() -> Self {
        Self {
            content_per_file: BTreeMap::new(),
        }
    }
    /// Sets the constants file content
    pub fn set_constants(&mut self, s: String) {
        self.content_per_file.insert("constants.ts".to_string(), s);
    }
    /// Sets the types file content
    pub fn set_types(&mut self, s: String) {
        self.content_per_file.insert("types.ts".to_string(), s);
    }
    /// Sets the commands file content
    pub fn set_commands(&mut self, s: String) {
        self.content_per_file.insert("commands.ts".to_string(), s);
    }
    /// Sets the events file content
    pub fn set_events(&mut self, s: String) {
        self.content_per_file.insert("events.ts".to_string(), s);
    }
    /// Sets the globals file content
    pub fn set_globals(&mut self, s: String) {
        self.content_per_file.insert("globals.ts".to_string(), s);
    }
}

/// Implemented for all languages which Tauri Specta supports exporting to.
///
/// Currently implemented for:
///  - [`specta_typescript::Typescript`]
///  - [`specta_jsdoc::JSDoc`]
pub trait LanguageExt {
    /// TODO
    type Error: std::error::Error + From<std::io::Error>;

    /// render the bindings file
    fn render(&self, cfg: &ExportContext) -> Result<String, Self::Error>;

    /// TODO
    fn format(&self, path: &Path) -> Result<(), Self::Error>;

    /// render the bindings per file
    fn render_per_file(&self, cfg: &ExportContext) -> Result<ExportFiles, Self::Error>;
}

impl<L: LanguageExt> LanguageExt for &L {
    type Error = L::Error;

    fn render(&self, cfg: &ExportContext) -> Result<String, Self::Error> {
        (*self).render(cfg)
    }

    fn format(&self, path: &Path) -> Result<(), Self::Error> {
        (*self).format(path)
    }
    
    fn render_per_file(&self, cfg: &ExportContext) -> Result<ExportFiles, Self::Error> {
        (*self).render_per_file(cfg)
    }
}

#[allow(unused)]
pub(crate) enum ItemType {
    Event,
    Command,
}

pub(crate) fn apply_as_prefix(plugin_name: &str, s: &str, item_type: ItemType) -> String {
    format!(
        "plugin:{}{}{}",
        plugin_name,
        match item_type {
            ItemType::Event => ":",
            ItemType::Command => "|",
        },
        s,
    )
}

/// The mode which the error handling is done in the bindings.
#[derive(Debug, Default, Copy, Clone)]
pub enum ErrorHandlingMode {
    /// Errors will be thrown
    Throw,
    /// Errors will be returned as a Result enum
    #[default]
    Result,
}

#[doc(hidden)]
pub mod internal {
    //! Internal logic for Tauri Specta.
    //! Nothing in this module has to conform to semver so it should not be used outside of this crate.
    //! It has to be public so macro's can access it.

    use super::*;

    /// called by `collect_commands` to construct `Commands`
    pub fn command<R: Runtime, F>(
        f: F,
        types: fn(&mut TypeMap) -> Vec<datatype::Function>,
        module_paths: Vec<&'static str>,
    ) -> Commands<R>
    where
        F: Fn(Invoke<R>) -> bool + Send + Sync + 'static,
    {
        Commands(Arc::new(f), types, module_paths)
    }

    /// called by `collect_events` to register events to an `Events`
    pub fn register_event<E: Event>(Events(events): &mut Events) {
        if events
            .insert(E::NAME, |type_map| {
                (E::sid(), E::reference(type_map, &[]).inner)
            })
            .is_some()
        {
            panic!("Another event with name {} is already registered!", E::NAME)
        }
    }
}
