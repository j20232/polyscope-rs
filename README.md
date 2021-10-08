# polyscope-rs: Rust bindings for polyscope

![Teaser](https://github.com/j20232/polyscope-rs/blob/main/assets/teaser/dynamic.gif)

## About

Wrapper of [polyscope](https://github.com/nmwsharp/polyscope)

(under development)

**Not supported for Linux/OS X** because I don't have PCs.... :)

## Features

- [x] Basic functions
   - [x] `init()`, `show()`
   - [x] Function to register user-defined callbacks
   - [ ] Functions to generate [imgui](https://github.com/ocornut/imgui) components
- [x] Visualization of positions, scalars, vectors and colors of point clouds
- [ ] Visualization of positions, scalars, vectors and colors of surface meshes
- [ ] Visualization of positions, scalars, vectors and colors of curve networks
- [ ] Visualization of positions, scalars, vectors and colors of volume meshes
- [ ] Support for linear algebra libraries
  - [ ] [nalgebra](https://github.com/dimforge/nalgebra)
  - [ ] [ndarray](https://github.com/rust-ndarray/ndarray)

## Installation

```sh
git clone https://github.com/j20232/polyscope-rs --recursive
cargo run --example 01_visualize_obj
```

or

```sh
cargo install polyscope
```

## Special thanks

- All contributors of the original [polyscope](https://github.com/nmwsharp/polyscope/graphs/contributors)

## License

MIT
