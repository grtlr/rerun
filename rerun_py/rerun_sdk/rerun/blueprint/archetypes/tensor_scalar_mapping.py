# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/tensor_scalar_mapping.fbs".

# You can extend this class by creating a "TensorScalarMappingExt" class in "tensor_scalar_mapping_ext.py".

from __future__ import annotations

from typing import Any

from attrs import define, field

from ... import components, datatypes
from ..._baseclasses import (
    Archetype,
)
from ...error_utils import catch_and_log_exceptions

__all__ = ["TensorScalarMapping"]


@define(str=False, repr=False, init=False)
class TensorScalarMapping(Archetype):
    """**Archetype**: Configures how tensor scalars are mapped to color."""

    def __init__(
        self: Any,
        *,
        mag_filter: components.MagnificationFilterLike | None = None,
        colormap: components.ColormapLike | None = None,
        gamma: datatypes.Float32Like | None = None,
    ):
        """
        Create a new instance of the TensorScalarMapping archetype.

        Parameters
        ----------
        mag_filter:
            Filter used when zooming in on the tensor.

            Note that the filter is applied to the scalar values *before* they are mapped to color.
        colormap:
            How scalar values map to colors.
        gamma:
            Gamma exponent applied to normalized values before mapping to color.

            Raises the normalized values to the power of this value before mapping to color.
            Acts like an inverse brightness. Defaults to 1.0.

            The final value for display is set as:
            `colormap( ((value - data_display_range.min) / (data_display_range.max - data_display_range.min)) ** gamma )`

        """

        # You can define your own __init__ function as a member of TensorScalarMappingExt in tensor_scalar_mapping_ext.py
        with catch_and_log_exceptions(context=self.__class__.__name__):
            self.__attrs_init__(mag_filter=mag_filter, colormap=colormap, gamma=gamma)
            return
        self.__attrs_clear__()

    def __attrs_clear__(self) -> None:
        """Convenience method for calling `__attrs_init__` with all `None`s."""
        self.__attrs_init__(
            mag_filter=None,  # type: ignore[arg-type]
            colormap=None,  # type: ignore[arg-type]
            gamma=None,  # type: ignore[arg-type]
        )

    @classmethod
    def _clear(cls) -> TensorScalarMapping:
        """Produce an empty TensorScalarMapping, bypassing `__init__`."""
        inst = cls.__new__(cls)
        inst.__attrs_clear__()
        return inst

    mag_filter: components.MagnificationFilterBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.MagnificationFilterBatch._optional,  # type: ignore[misc]
    )
    # Filter used when zooming in on the tensor.
    #
    # Note that the filter is applied to the scalar values *before* they are mapped to color.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    colormap: components.ColormapBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.ColormapBatch._optional,  # type: ignore[misc]
    )
    # How scalar values map to colors.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    gamma: components.GammaCorrectionBatch | None = field(
        metadata={"component": "optional"},
        default=None,
        converter=components.GammaCorrectionBatch._optional,  # type: ignore[misc]
    )
    # Gamma exponent applied to normalized values before mapping to color.
    #
    # Raises the normalized values to the power of this value before mapping to color.
    # Acts like an inverse brightness. Defaults to 1.0.
    #
    # The final value for display is set as:
    # `colormap( ((value - data_display_range.min) / (data_display_range.max - data_display_range.min)) ** gamma )`
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    __str__ = Archetype.__str__
    __repr__ = Archetype.__repr__  # type: ignore[assignment]
