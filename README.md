# EFM32PG12 PAC

Low-level register mappings for the [Silicon Labs EFM32PG12] family of ARM Cortex-M4 microcontrollers, written in Rust. The code is generated automatically from a vendor-supplied SVD file, using [svd2rust].

The purpose of this crate is to give embedded programs or libraries written Rust access to the complete functionality of EFM32PG12 MCUs.

## Documentation

SVD files are available in the [EFM32PG12 CMSIS-Pack].
All devices in this family share the same register description yet there is one SVD file per device in the pack.
Take only one of those SVD files and remove the device specific parts of the `<name>` and `<description>` tags.

Additional vendor supplied documents:
- [Datasheet](https://www.silabs.com/documents/public/data-sheets/efm32pg12-datasheet.pdf)
- [Reference Manual](https://www.silabs.com/documents/public/reference-manuals/efm32pg12-rm.pdf)
- [Errata](https://www.silabs.com/documents/public/errata/efm32pg12-errata.pdf)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[Silicon Labs EFM32PG12]: https://www.silabs.com/products/mcu/32-bit/efm32-pearl-gecko
[EFM32PG12 CMSIS-Pack]: https://www.silabs.com/documents/public/cmsis-packs/SiliconLabs.EFM32PG12B_DFP.5.8.2.pack
