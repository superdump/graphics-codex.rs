# graphics-codex.rs

An implementation of [Morgan McGuire (@morgan3d)](@morgan3d)'s graphicscodex.com in Rust.

## Usage

```
graphics-codex 0.1.0
Robert Swain <robert.swain@gmail.com>
An implementation of Morgan McGuire's graphicscodex.com in Rust

USAGE:
    ray_casting.exe [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <HEIGHT>    Rendered / displayed height
        --obj <OBJ>          Wavefront OBJ file to load
    -y, --vfov <VFOV>        Vertical field of view
    -w, --width <WIDTH>      Rendered / displayed width
    -z, --znear <ZNEAR>      Distance from the camera to the image/near-clipping plane
```

e.g. `cargo run --release --example ray_casting -- -w 160 -h 90`

## License

MIT. See LICENSE for more details.
