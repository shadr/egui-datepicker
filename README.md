# egui-datepicker
[![Latest version](https://img.shields.io/crates/v/egui-datepicker)](https://crates.io/crates/egui-datepicker)

This library provide a simple date picker widget for egui with some customization. Checkout the [gif](media/preview.gif) to see widget in action!

<p align="center">
    <img src="media/datepicker-image.png">
</p>

## ⚡️ Quickstart

Add `egui-datepicker` as dependency to your project
```toml
[dependencies]
egui-datepicker = "0.2"
```

Import necessary structs
```rust
use egui_datepicker::{DatePicker, Date, Utc};
```

or if you already include `chrono` in your project
```rust
use egui_datepicker::DatePicker;
use chrono::{Date, offset::Utc};
```

Add date field with selected time offset in app struct
```rust
struct MyApp {
    date: Date<Utc>,
}
```

Add widget in update function
```rust
fn update(/*snip*/) {
    /*snip*/
    ui.add(DatePicker::new("datepicker-unique-id", &mut self.date));
    /*snip*/
}
```

## 👀 Customization
You can set first day of week to sunday with
```rust
DatePicker::new(/*snip*/).sunday_first(true)
```
Make popup window movable
```rust
DatePicker::new(/*snip*/).movable(true)
```
Set different date format
```rust
DatePicker::new(/*snip*/).date_format("%d/%m/%Y")
```

## ⚠️ License

`egui-datepicker` is licensed under MIT OR Apache-2.0
