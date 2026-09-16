# Generic Chunk Key Encoding for Zarr v3

This crate provides an implementation of the proposed generic/ template/ format [chunk key encoding](https://zarr-specs.readthedocs.io/en/v3.1.0/v3/chunk-key-encodings/index.html) for [zarr](https://zarr.dev/) v3.

The specification (WIP) is here: <https://github.com/zarr-developers/zarr-extensions/pull/69>.

## Features

The `zarrs` feature (default) enables use within the [zarrs](https://crates.io/crates/zarrs) ecosystem.

## Bindings

There are python bindings in the `python/bindings` directory, which are designed for use with the [zarr](https://pypi.org/project/zarr/) package.
