---
title: "LeafTransforms3D"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/mod.rs -->

One or more transforms between the parent and the current entity which are *not* propagated in the transform hierarchy.

For transforms that are propagated in the transform hierarchy, see [`archetypes.Transform3D`](https://rerun.io/docs/reference/types/archetypes/transform3d).

If both [`archetypes.LeafTransforms3D`](https://rerun.io/docs/reference/types/archetypes/leaf_transforms3d) and [`archetypes.Transform3D`](https://rerun.io/docs/reference/types/archetypes/transform3d) are present,
first the tree propagating [`archetypes.Transform3D`](https://rerun.io/docs/reference/types/archetypes/transform3d) is applied, then [`archetypes.LeafTransforms3D`](https://rerun.io/docs/reference/types/archetypes/leaf_transforms3d).

Currently, most visualizers support only a single leaf transform per entity.
Check archetype documentations for details - if not otherwise specified, only the first leaf transform is applied.

From the point of view of the entity's coordinate system,
all components are applied in the inverse order they are listed here.
E.g. if both a translation and a max3x3 transform are present,
the 3x3 matrix is applied first, followed by the translation.

## Components

**Optional**: [`LeafTranslation3D`](../components/leaf_translation3d.md), [`LeafRotationAxisAngle`](../components/leaf_rotation_axis_angle.md), [`LeafRotationQuat`](../components/leaf_rotation_quat.md), [`LeafScale3D`](../components/leaf_scale3d.md), [`LeafTransformMat3x3`](../components/leaf_transform_mat3x3.md)

## Shown in
* [Spatial3DView](../views/spatial3d_view.md)
* [Spatial2DView](../views/spatial2d_view.md) (if logged above active projection)

## API reference links
 * 🌊 [C++ API docs for `LeafTransforms3D`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1archetypes_1_1LeafTransforms3D.html)
 * 🐍 [Python API docs for `LeafTransforms3D`](https://ref.rerun.io/docs/python/stable/common/archetypes#rerun.archetypes.LeafTransforms3D)
 * 🦀 [Rust API docs for `LeafTransforms3D`](https://docs.rs/rerun/latest/rerun/archetypes/struct.LeafTransforms3D.html)

## Example

### Regular & leaf transform in tandom

snippet: archetypes/leaf_transforms3d_combined

<picture data-inline-viewer="snippets/leaf_transforms3d_combined">
  <source media="(max-width: 480px)" srcset="https://static.rerun.io/leaf_transform3d/41674f0082d6de489f8a1cd1583f60f6b5820ddf/480w.png">
  <source media="(max-width: 768px)" srcset="https://static.rerun.io/leaf_transform3d/41674f0082d6de489f8a1cd1583f60f6b5820ddf/768w.png">
  <source media="(max-width: 1024px)" srcset="https://static.rerun.io/leaf_transform3d/41674f0082d6de489f8a1cd1583f60f6b5820ddf/1024w.png">
  <source media="(max-width: 1200px)" srcset="https://static.rerun.io/leaf_transform3d/41674f0082d6de489f8a1cd1583f60f6b5820ddf/1200w.png">
  <img src="https://static.rerun.io/leaf_transform3d/41674f0082d6de489f8a1cd1583f60f6b5820ddf/full.png">
</picture>
