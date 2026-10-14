//! # DW14 — Vending Machine Item
//!
//! ```text
//! A1 Pretzels 125
//! ```
//!
//! Do not edit `src/main.rs`; the bugs are in the library files.

/// One snack in the vending machine.
pub struct Item {
    pub code: String,
    pub name: String
    pub price_cents: u32,
}

/// Builds the item that sits in slot A1.
///
/// ```
/// let item = dw14::make_item();
/// assert_eq!(item.code, "A1");
/// assert_eq!(item.price_cents, 125);
/// ```
pub fn make_item() -> Item {
    Item {
        code: String::from("A1"),
        name: String::from("Pretzels"),
        price_cents: "125",
    }
}

/// Formats an item as `code name price`.
///
/// ```
/// let item = dw14::make_item();
/// assert_eq!(dw14::describe(&item), "A1 Pretzels 125");
/// ```
pub fn describe(item: &Item) -> String {
    format!("{} {} {}", item.code, item.nmae, item.price_cents)
}
