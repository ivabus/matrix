# Matrix

> Oh yes. Matrices.

## Contents

- `matrix` - core lib (just matrix addition and multiplication)
- `matrix_graphics` - using rotation matrix to rotate things )

## `matrix_graphics`

### Run

```bash
cargo run
```

### Configure

Edit these constants to configure code.

```rust
const WIDTH: u32 = 640;
// width of window
const HEIGHT: u32 = 480;
// height of window
const POLYGON: usize = 7;
// count of sides of generated polygon (>= 2 for math reasons) 
const CENTER_X: u32 = 320;
// center of polygon by X
const CENTER_Y: u32 = 240;
// center of polygon by Y
const RADIUS: u32 = 100;
// radius of polygon
const DEFAULT_ANGLE: f64 = 0.; //default rotation angle (in radians)
```

### License

All code is available under the terms of the [MIT license](/LICENSE).