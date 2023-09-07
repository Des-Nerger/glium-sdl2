# glium_sdl2

An SDL2 backend for [Glium](https://github.com/glium/glium) - a high-level
OpenGL wrapper for the Rust language.

```toml
[dependencies]
glium_sdl2 = { git = "https://github.com/Des-Nerger/glium-sdl2" }
sdl2 = "0"
glium = { version = "0", default-features = false }
```

glium_sdl2 doesn't reexport the `glium` or `sdl2` crates, so you must declare
them (with versions compatible to those of glium_sdl2) in your `Cargo.toml` file.

glium_sdl2's version will be bumped once this library, `glium` or `sdl2`
make breaking changes.

## Example usage

See the examples/ folder for examples. Here's a bare-bones skeleton program that initializes SDL2 and Glium, and does nothing:
* [blank.rs](examples/blank.rs)

Using glium with SDL2 is very similar to using glium with glutin.
Generally speaking, Glium documentation and tutorials should be fairly trivial
to adapt to SDL2.

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
