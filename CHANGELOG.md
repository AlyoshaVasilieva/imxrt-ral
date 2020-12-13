# Changelog

## [Unreleased]

* **BREAKING** The RAL depends on `cortex-m`, version `0.7`. All `Interrupt`
  enumerations now implement `cortex_m::interrupt::InterruptNumber`, instead
  of `bare_metal::Nr`.

## [0.4.0] 2020-08-29

* **BREAKING** The RAL's `"rtfm"` feature is changed to `"rtic"`, reflecting the framework's
  new name. Users who are relying on the `"rtfm"` feature should now use the `"rtic"` feature.

## [0.3.0] 2020-06-18

* Only emit link section for `__INTERRUPTS` when compiling for ARM targets
* Fix RAL's documentation to refer to i.MX RT registers

## [0.2.1] 2020-04-10

* Fixes cargo release, adds release building documentation

## [0.2.0] 2020-04-08

* Port of ccm, iomuxc, uart, i2c, and spi peripherals from teensy4-rs!
* Support for imxrt1060evk board as well as teensy4

## [0.1.0] 2020-02-06

Initial build and release of imxrt family of peripheral access crates

[Unreleased]: https://github.com/imxrt-rs/imxrt-rs/compare/0.4.0...HEAD
[0.4.0]: https://github.com/imxrt-rs/imxrt-rs/compare/0.3.0...0.4.0
[0.3.0]: https://github.com/imxrt-rs/imxrt-rs/compare/0.2.1...0.3.0
[0.2.1]: https://github.com/imxrt-rs/imxrt-rs/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/imxrt-rs/imxrt-rs/compare/0.1.0...0.2.1
[0.1.0]: https://github.com/imxrt-rs/imxrt-rs/releases/tag/0.1.0