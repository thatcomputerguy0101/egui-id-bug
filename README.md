# egui ID bug

This demonstrates an ID duplication bug in `egui` when used in conjunction with `three-d`. I was unable to reproduce this when using `eframe` instead of `three-d`, as can be seen on the [eframe branch](0a5c3d7368133e145c6be2c9ed02e2bdf93706e4). This bug seems to result in duplicate IDs between container widgets that share the same parent, such as `horizontal` or `collapsing`. In the example, there should be a tooltip shown whenever the first `horizontal` contains the pointer. However, that tooltip does not show up if another container widget is present, and will instead only show up when hovering over the last container widget within the parent container.

## Running

```bash
cargo run
```
