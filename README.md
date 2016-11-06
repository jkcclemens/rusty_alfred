# rusty_alfred

![docs](https://docs.rs/rusty_alfred/badge.svg)

Rusty Alfred is a quick library built to interact with [Alfred 3](http://alfredapp.com/) via
[Script Filters](https://www.alfredapp.com/help/workflows/inputs/script-filter/) in the
[JSON format](https://www.alfredapp.com/help/workflows/inputs/script-filter/json/).

Here's a quick example, but there are more in the `bin` directory.

```rust
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
```

The above example produces
![example output](http://i.imgur.com/UcOIyNa.png)
