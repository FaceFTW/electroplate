# Electroplate Test Case List

This is more internal documentation for me to ensure I have the integration cases defined, implemented, and passing. It also provides some look into what you should be able to do with Electroplate.

[`trybuild`](https://github.com/dtolnay/trybuild) is used as the harness, so each case is defined as a single file that will either compile and run successfully (assertions in `main()`) OR fail with some kind of compiler error.

## Getters - `/tests/get`
- [ ] Getter - Field Type implements `Clone`/`Copy`
- [ ] Getter - Field Type does not implement `Copy` (i.e. `String`)
- [ ] Getter - Field Type `Box<T: Clone || Copy>`
- [ ] Getter - Field Type `Box<T: !Clone && !Copy>`
- [ ] Getter - Field Type is Sized Array `[T : Clone || Copy; <usize>]`
- [ ] Getter - Field Type is Sized Array `[T : !Clone && !Copy; <usize>]`
- [ ] Getter

## Reference Getters - `/tests/ref_get`

## Mutable Reference Getters - `/tests/mut_ref_get`

## Mixed Getter Types - `/tests/mixed_get`

## Immutable Setters - `/tests/set`

## Mutable Setters - `/tests/mut/set`

## Constructors - TBD

## Negative Examples - `/tests/negative_examples`

These represent instances where the macro should fail to compile, but fall outside of general functionality

- [ ]