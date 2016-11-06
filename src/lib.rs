//! Rusty Alfred is a quick library built to interact with [Alfred 3](http://alfredapp.com/) via
//! [Script Filters](https://www.alfredapp.com/help/workflows/inputs/script-filter/) in the
//! [JSON format](https://www.alfredapp.com/help/workflows/inputs/script-filter/json/).
//!
//! Basic example:
//!
//! ```
//! use rusty_alfred::*;
//!
//! fn main() {
//!   let items = AlfredItems::new()
//!     .item(AlfredItem::new("First item")
//!       .subtitle("The first item's subtitle"))
//!     .item(AlfredItem::new("Second item")
//!       .subtitle("Another subtitle!"));
//!   println!("{}", items.to_json().unwrap());
//! }
//! ```
//!

#![feature(proc_macro)]

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;

/// Errors using [error-chain](https://crates.io/crates/error_chain).
pub mod errors;

use errors::*;

/// The parent for all Alfred items. This is what should be printed to `stdout` via `to_json()` for
/// Alfred to display results.
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AlfredItems {
  /// The items to be displayed in Alfred.
  pub items: Vec<AlfredItem>
}

impl Default for AlfredItems {
  fn default() -> Self {
    AlfredItems {
      items: Vec::new()
    }
  }
}

impl AlfredItems {
  /// Makes an empty items container.
  pub fn new() -> Self {
    AlfredItems::default()
  }

  /// Adds an item to this container. This retains the order of items, but Alfred may not display
  /// them in that order if the items have `uid`s.
  pub fn item(mut self, item: AlfredItem) -> Self {
    self.items.push(item);
    self
  }

  /// Attempts to use serde to convert this container to JSON. The resulting JSON is ready to be
  /// given to Alfred.
  pub fn to_json(&self) -> Result<String> {
    serde_json::to_string(self).chain_err(|| "could not serialize AlfredItems")
  }
}

/// An item to be displayed in Alfred. Only the `title` attribute is required.
///
/// Documentation mostly copied from
/// [here](https://www.alfredapp.com/help/workflows/inputs/script-filter/json/).
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AlfredItem {
  /// This is a unique identifier for the item which allows help Alfred to learn about this item for
  /// subsequent sorting and ordering of the user's actioned results.
  ///
  /// It is important that you use the same UID throughout subsequent executions of your script to
  /// take advantage of Alfred's knowledge and sorting. If you would like Alfred to always show the
  /// results in the order you return them from your script, exclude the UID field.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub uid: Option<String>,
  /// The title displayed in the result row. There are no options for this element and it is
  /// essential that this element is populated.
  pub title: String,
  /// The subtitle displayed in the result row. This element is optional.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subtitle: Option<String>,
  /// The argument which is passed through the workflow to the connected output action.
  ///
  /// While the arg attribute is optional, it's highly recommended that you populate this as it's
  /// the string which is passed to your connected output actions. If excluded, you won't know which
  /// result item the user has selected.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arg: Option<String>,
  /// The icon displayed in the result row. Workflows are run from their workflow folder,
  /// so you can reference icons stored in your workflow relatively.
  ///
  /// See the documentation for `AlfredItemIcon` for more information.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub icon: Option<AlfredItemIcon>,
  /// If this item is valid or not. If an item is valid then Alfred will action this item when the
  /// user presses return. If the item is not valid, Alfred will do nothing. This allows you to
  /// intelligently prevent Alfred from actioning a result based on the current {query} passed into
  /// your script.
  ///
  /// If you exclude the valid attribute, Alfred assumes that your item is valid.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valid: Option<bool>,
  /// An optional but recommended string you can provide which is populated into Alfred's search
  /// field if the user auto-complete's the selected result (⇥ by default).
  ///
  /// If the item is set as "valid": false, the auto-complete text is populated into Alfred's search
  /// field when the user actions the result.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub autocomplete: Option<String>,
  /// The type of item. See the documentation for `AlfredItemType` for more information.
  #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
  pub item_type: Option<AlfredItemType>,
  /// The mod element gives you control over how the modifier keys react. See the documentation for
  /// `AlfredItemMods` for more information.
  #[serde(skip_serializing_if = "Option::is_none", rename = "mods")]
  pub item_mods: Option<AlfredItemMods>,
  /// The text element defines the text the user will get when copying the selected result row with
  /// ⌘C or displaying large type with ⌘L. See the `AlfredItemText` documentation for more
  /// information.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<AlfredItemText>,
  /// A Quick Look URL which will be visible if the user uses the Quick Look feature within Alfred
  /// (tapping shift, or cmd+y)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub quicklookurl: Option<String>
}

impl AlfredItem {
  /// Makes a new item with a title.
  ///
  /// Use the builder methods to customize the item more.
  pub fn new<T>(title: T) -> Self
    where T: AsRef<str>
  {
    AlfredItem {
      uid: None,
      title: title.as_ref().to_owned(),
      subtitle: None,
      arg: None,
      icon: None,
      valid: None,
      autocomplete: None,
      item_type: None,
      item_mods: None,
      text: None,
      quicklookurl: None
    }
  }

  /// This is a unique identifier for the item which allows help Alfred to learn about this item for
  /// subsequent sorting and ordering of the user's actioned results.
  ///
  /// It is important that you use the same UID throughout subsequent executions of your script to
  /// take advantage of Alfred's knowledge and sorting. If you would like Alfred to always show the
  /// results in the order you return them from your script, exclude the UID field.
  pub fn uid<T>(mut self, uid: T) -> Self
    where T: AsRef<str>
  {
    self.uid = Some(uid.as_ref().to_owned());
    self
  }

  /// The title displayed in the result row. There are no options for this element and it is
  /// essential that this element is populated.
  pub fn title<T>(mut self, title: T) -> Self
    where T: AsRef<str>
  {
    self.title = title.as_ref().to_owned();
    self
  }

  /// The subtitle displayed in the result row. This element is optional.
  pub fn subtitle<T>(mut self, subtitle: T) -> Self
    where T: AsRef<str>
  {
    self.subtitle = Some(subtitle.as_ref().to_owned());
    self
  }

  /// The argument which is passed through the workflow to the connected output action.
  ///
  /// While the arg attribute is optional, it's highly recommended that you populate this as it's
  /// the string which is passed to your connected output actions. If excluded, you won't know which
  /// result item the user has selected.
  pub fn arg<T>(mut self, arg: T) -> Self
    where T: AsRef<str>
  {
    self.arg = Some(arg.as_ref().to_owned());
    self
  }

  /// The icon displayed in the result row. Workflows are run from their workflow folder,
  /// so you can reference icons stored in your workflow relatively.
  ///
  /// See the documentation for `AlfredItemIcon` for more information.
  pub fn icon(mut self, icon: AlfredItemIcon) -> Self {
    self.icon = Some(icon);
    self
  }

  /// If this item is valid or not. If an item is valid then Alfred will action this item when the
  /// user presses return. If the item is not valid, Alfred will do nothing. This allows you to
  /// intelligently prevent Alfred from actioning a result based on the current {query} passed into
  /// your script.
  ///
  /// If you exclude the valid attribute, Alfred assumes that your item is valid.
  pub fn valid(mut self, valid: bool) -> Self {
    self.valid = Some(valid);
    self
  }

  /// An optional but recommended string you can provide which is populated into Alfred's search
  /// field if the user auto-complete's the selected result (⇥ by default).
  ///
  /// If the item is set as "valid": false, the auto-complete text is populated into Alfred's search
  /// field when the user actions the result.
  pub fn autocomplete<T>(mut self, autocomplete: T) -> Self
    where T: AsRef<str>
  {
    self.autocomplete = Some(autocomplete.as_ref().to_owned());
    self
  }

  /// The type of item. See the documentation for `AlfredItemType` for more information.
  pub fn item_type(mut self, item_type: AlfredItemType) -> Self {
    self.item_type = Some(item_type);
    self
  }

  /// The text element defines the text the user will get when copying the selected result row with
  /// ⌘C or displaying large type with ⌘L. See the `AlfredItemText` documentation for more
  /// information.
  pub fn item_mods(mut self, item_mods: AlfredItemMods) -> Self {
    self.item_mods = Some(item_mods);
    self
  }

  /// Builder method to set the text field for this item.
  pub fn text(mut self, text: AlfredItemText) -> Self {
    self.text = Some(text);
    self
  }

  /// A Quick Look URL which will be visible if the user uses the Quick Look feature within Alfred
  /// (tapping shift, or cmd+y)
  pub fn quicklookurl<T>(mut self, quicklookurl: T) -> Self
    where T: AsRef<str>
  {
    self.quicklookurl = Some(quicklookurl.as_ref().to_owned());
    self
  }
}

/// Types for the `type` field on an `AlfredItemType`.
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum AlfredItemType {
  /// Tells Alfred to treat this item as a normal action. This is the default if not specified.
  #[serde(rename = "default")]
  Default,
  /// Tells Alfred to treat this item as a file.
  ///
  /// By specifying "type": "file", this makes Alfred treat your result as a file on your system.
  /// This allows the user to perform actions on the file like they can with Alfred's standard file
  /// filters.
  #[serde(rename = "file")]
  File,
  /// Tells Alfred to treat this item as a file but to also skip checking if the file exists.
  ///
  /// When returning files, Alfred will check if the file exists before presenting that result to
  /// the user. This has a very small performance implication but makes the results as predictable
  /// as possible. If you would like Alfred to skip this check as you are certain that the files you
  /// are returning exist, you can use "type": "file:skipcheck".
  #[serde(rename = "file:skipcheck")]
  FileSkipCheck
}

/// Item icon information.
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AlfredItemIcon {
  /// The type of icon this is. Defaults to `None`.
  ///
  /// By omitting the "type", Alfred will load the file path itself, for example a png. By using
  /// "type": "fileicon", Alfred will get the icon for the specified path. Finally, by using "type":
  /// "filetype", you can get the icon of a specific file, for example "path": "public.png"
  #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
  pub icon_type: Option<AlfredItemIconType>,
  /// The path to the icon for this item, either absolute or relative to the workflow folder.
  pub path: String
}

impl AlfredItemIcon {
  /// Creates an item icon pointing to `path`.
  ///
  /// Use the builder methods for more customization.
  pub fn new<T>(path: T) -> Self
    where T: AsRef<str>
  {
    AlfredItemIcon {
      icon_type: None,
      path: path.as_ref().to_owned()
    }
  }

  /// The type of icon this is. Defaults to `None`.
  ///
  /// By omitting the "type", Alfred will load the file path itself, for example a png. By using
  /// "type": "fileicon", Alfred will get the icon for the specified path. Finally, by using "type":
  /// "filetype", you can get the icon of a specific file, for example "path": "public.png"
  pub fn icon_type(mut self, icon_type: AlfredItemIconType) -> Self {
    self.icon_type = Some(icon_type);
    self
  }

  /// The path to the icon for this item, either absolute or relative to the workflow folder.
  pub fn path<T>(mut self, path: T) -> Self
    where T: AsRef<str>
  {
    self.path = path.as_ref().to_owned();
    self
  }
}

/// The type of icon for an item.
///
/// The icon displayed in the result row. Workflows are run from their workflow folder, so you can
/// reference icons stored in your workflow relatively.
///
/// By omitting the "type", Alfred will load the file path itself, for example a png. By using
/// "type": "fileicon", Alfred will get the icon for the specified path. Finally, by using "type":
/// "filetype", you can get the icon of a specific file, for example "path": "public.png"
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum AlfredItemIconType {
  /// By using "type": "fileicon", Alfred will get the icon for the specified path.
  #[serde(rename = "fileicon")]
  FileIcon,
  /// By using "type": "filetype", you can get the icon of a specific file, for example "path":
  /// "public.png"
  #[serde(rename = "filetype")]
  FileType
}

/// The information for the `text` field on an item.
///
/// If these are not defined, you will inherit Alfred's standard behaviour where the arg is copied
/// to the Clipboard or used for Large Type.
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AlfredItemText {
  /// This defines the text the user will get when copying the selected result row with ⌘C.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub copy: Option<String>,
  /// This defines the text the user will get when displaying large type with ⌘L.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub largetype: Option<String>
}

impl Default for AlfredItemText {
  fn default() -> Self {
    AlfredItemText {
      copy: None,
      largetype: None
    }
  }
}

impl AlfredItemText {
  /// Creates a default item text.
  ///
  /// Use the builder methods for more customization.
  pub fn new() -> Self {
    AlfredItemText::default()
  }

  /// This defines the text the user will get when copying the selected result row with ⌘C.
  pub fn copy<T>(mut self, copy: T) -> Self
    where T: AsRef<str>
  {
    self.copy = Some(copy.as_ref().to_owned());
    self
  }

  /// This defines the text the user will get when displaying large type with ⌘L.
  pub fn largetype<T>(mut self, largetype: T) -> Self
    where T: AsRef<str>
  {
    self.largetype = Some(largetype.as_ref().to_owned());
    self
  }
}

/// Information about the alternate functions of an item.
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AlfredItemMods {
  /// The modifier information for when Alt/Option is being pressed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub alt: Option<AlfredItemMod>,
  /// The modifier information for when Command is being pressed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cmd: Option<AlfredItemMod>,
  /// The modifier information for when Control is being pressed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub ctrl: Option<AlfredItemMod>,
  /// The modifier information for when Shift is being pressed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub shift: Option<AlfredItemMod>
}

impl Default for AlfredItemMods {
  fn default() -> Self {
    AlfredItemMods {
      alt: None,
      cmd: None,
      ctrl: None,
      shift: None
    }
  }
}

impl AlfredItemMods {
  /// Creates an empty, default struct.
  ///
  /// Use the builder methods for more customization.
  pub fn new() -> Self {
    AlfredItemMods::default()
  }

  /// The modifier information for when Alt/Option is being pressed.
  pub fn alt(mut self, alt: AlfredItemMod) -> Self {
    self.alt = Some(alt);
    self
  }

  /// The modifier information for when Command is being pressed.
  pub fn cmd(mut self, cmd: AlfredItemMod) -> Self {
    self.cmd = Some(cmd);
    self
  }

  /// The modifier information for when Control is being pressed.
  pub fn ctrl(mut self, ctrl: AlfredItemMod) -> Self {
    self.ctrl = Some(ctrl);
    self
  }

  /// The modifier information for when Shift is being pressed.
  pub fn shift(mut self, shift: AlfredItemMod) -> Self {
    self.shift = Some(shift);
    self
  }
}

/// Information about a specific modifier to an item.
#[derive(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub struct AlfredItemMod {
  /// Marks if the result is valid based on the modifier selection.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub valid: Option<bool>,
  /// The arg to be passed out if actioned with the modifier.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub arg: Option<String>,
  /// The subtitle to be displayed while the modifier is pressed.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subtitle: Option<String>
}

impl Default for AlfredItemMod {
  fn default() -> Self {
    AlfredItemMod {
      valid: None,
      arg: None,
      subtitle: None
    }
  }
}

impl AlfredItemMod {
  /// Creates an empty item modifier struct.
  ///
  /// Use the builder methods for further customization.
  pub fn new() -> Self {
    AlfredItemMod::default()
  }

  /// Marks if the result is valid based on the modifier selection.
  pub fn valid(mut self, valid: bool) -> Self {
    self.valid = Some(valid);
    self
  }

  /// The arg to be passed out if actioned with the modifier.
  pub fn arg<T>(mut self, arg: T) -> Self
    where T: AsRef<str>
  {
    self.arg = Some(arg.as_ref().to_owned());
    self
  }

  /// The subtitle to be displayed while the modifier is pressed.
  pub fn subtitle<T>(mut self, subtitle: T) -> Self
    where T: AsRef<str>
  {
    self.subtitle = Some(subtitle.as_ref().to_owned());
    self
  }
}
