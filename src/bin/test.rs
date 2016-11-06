extern crate rusty_alfred;

use rusty_alfred::*;

fn main() {
  let some_items = AlfredItems::new()
    .item(AlfredItem::new("Title 1")
      .subtitle("This is a subtitle for item 1.")
      .arg("title1")
      .icon(AlfredItemIcon::new("one.png"))
      .valid(true)
      .autocomplete("first")
      .item_type(AlfredItemType::Default)
      .item_mods(AlfredItemMods::default()
        .alt(AlfredItemMod::default()
          .subtitle("Secret option subtitle.")))
      .text(AlfredItemText::default()
        .copy("You copied option one.")
        .largetype("Hello, large type!"))
      .quicklookurl("https://google.com"))
    .item(AlfredItem::new("Title 2")
      .subtitle("This is a subtitle for item 2.")
      .arg("title2"));
  let json = some_items.to_json().unwrap();
  println!("{}", json);
}
