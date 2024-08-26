---
title: "PixelFormat"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

Specifieds a particular format of an [`archetypes.Image`](https://rerun.io/docs/reference/types/archetypes/image).

Most images can be described by a [`datatypes.ColorModel`](https://rerun.io/docs/reference/types/datatypes/color_model) and a [`datatypes.ChannelDatatype`](https://rerun.io/docs/reference/types/datatypes/channel_datatype),
e.g. `RGB` and `U8` respectively.

However, some image formats has chroma downsampling and/or
use differing number of bits per channel, and that is what this [`datatypes.PixelFormat`](https://rerun.io/docs/reference/types/datatypes/pixel_format) is for.

All these formats support random access.

For more compressed image formats, see [`archetypes.EncodedImage`](https://rerun.io/docs/reference/types/archetypes/encoded_image).

## Variants

* NV12
* YUY2

## API reference links
 * 🌊 [C++ API docs for `PixelFormat`](https://ref.rerun.io/docs/cpp/stable/namespacererun_1_1datatypes.html)
 * 🐍 [Python API docs for `PixelFormat`](https://ref.rerun.io/docs/python/stable/common/datatypes#rerun.datatypes.PixelFormat)
 * 🦀 [Rust API docs for `PixelFormat`](https://docs.rs/rerun/latest/rerun/datatypes/enum.PixelFormat.html)


## Used by

* [`ImageFormat`](../datatypes/image_format.md)