extern crate rusty_alfred;

use rusty_alfred::*;

fn main() {
  let items = AlfredItems::new()
    .item(AlfredItem::new("First item")
      .subtitle("The first item's subtitle"))
    .item(AlfredItem::new("Second item")
      .subtitle("Another subtitle!"));
  println!("{}", items.to_json().unwrap());
}
