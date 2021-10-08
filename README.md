# polyscope-rs: Rust bindings for polyscope

![Teaser](https://github.com/j20232/polyscope-rs/blob/main/assets/teaser/dynamic.gif)

## About

Wrapper of [polyscope](https://github.com/nmwsharp/polyscope)

(under development)

**Not supported for Linux/OS X** because I don't have PCs.... :)

## Features

- [x] Visualization of point clouds
  - [x] positions
  - [x] scalars
  - [x] vectors
  - [x] colors
- [ ] Visualization of surface meshes
  - [ ] positions
  - [ ] scalars
  - [ ] vectors
  - [ ] colors
- [ ] Visualization of curve networks
  - [ ] positions
  - [ ] scalars
  - [ ] vectors
  - [ ] colors
- [ ] Visualization of volume meshes
  - [ ] positions
  - [ ] scalars
  - [ ] vectors
  - [ ] colors

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
