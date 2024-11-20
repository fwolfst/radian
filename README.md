<div align="center">

# radian

A normalized angle type

[![crates.io](https://img.shields.io/crates/v/radian?style=for-the-badge)](https://crates.io/crates/radian)
[![docs.rs](https://img.shields.io/docsrs/radian?style=for-the-badge)](https://docs.rs/radian/latest/radian)

</div>

## Cargo Features

### `std`

The `std` feature is enabled by default. This uses the standard library implmentation of math functions like sin and cos.

### `libm`

The `libm` feature can be enabled instead of `std`. This will use the [`libm`](https://crates.io/crates/libm) crate for math functions like sin and cos.

### `ufmt`

The `ufmt` feature will add the [`ufmt::uDisplay`](https://docs.rs/ufmt/latest/ufmt/trait.uDisplay.html) implmentation for `Angle`.

