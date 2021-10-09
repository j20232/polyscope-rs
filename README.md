# polyscope-rs: Rust bindings for polyscope

![Teaser](https://github.com/j20232/polyscope-rs/blob/main/assets/teaser/dynamic.gif)

## About

Rust wrapper of [polyscope](https://github.com/nmwsharp/polyscope)

(under development)

- Currently, you can run `polyscope-rs` **only on Windows**
- **Not supported for Linux/OS X**
  - I don't have PCs.... :)

## Features

- [x] Basic functions
  - [x] `init()`, `show()`
  - [x] Function to register user-defined callbacks
  - [ ] Functions to generate [imgui](https://github.com/ocornut/imgui) components
- [ ] Visualization
  - [x] point clouds
  - [ ] surface meshes
  - [ ] curve networks
  - [ ] volume meshes
- [ ] Support for linear algebra libraries
  - [x] [nalgebra](https://github.com/dimforge/nalgebra): Only supported for `DMatrix`
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

## Contribute

Pull requests are welcome

## Special thanks

- All contributors of the original [polyscope](https://github.com/nmwsharp/polyscope/graphs/contributors)

## License

MIT
