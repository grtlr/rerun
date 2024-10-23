# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/datatypes/graph_location.fbs".

# You can extend this class by creating a "GraphLocationExt" class in "graph_location_ext.py".

from __future__ import annotations

from typing import Any, Sequence, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import (
    BaseBatch,
    BaseExtensionType,
)

__all__ = ["GraphLocation", "GraphLocationArrayLike", "GraphLocationBatch", "GraphLocationLike", "GraphLocationType"]


def _graph_location__entity_path__special_field_converter_override(x: datatypes.EntityPathLike) -> datatypes.EntityPath:
    if isinstance(x, datatypes.EntityPath):
        return x
    else:
        return datatypes.EntityPath(x)


def _graph_location__node_id__special_field_converter_override(x: datatypes.GraphNodeIdLike) -> datatypes.GraphNodeId:
    if isinstance(x, datatypes.GraphNodeId):
        return x
    else:
        return datatypes.GraphNodeId(x)


@define(init=False)
class GraphLocation:
    """
    **Datatype**: Uniquely identifies a node in a graph by its entity path and node id.

    We require this because the same node id can be used in multiple entities.
    """

    def __init__(self: Any, entity_path: datatypes.EntityPathLike, node_id: datatypes.GraphNodeIdLike):
        """
        Create a new instance of the GraphLocation datatype.

        Parameters
        ----------
        entity_path:
            The entity path that specifies where to find the node.
        node_id:
            The id of the node.

        """

        # You can define your own __init__ function as a member of GraphLocationExt in graph_location_ext.py
        self.__attrs_init__(entity_path=entity_path, node_id=node_id)

    entity_path: datatypes.EntityPath = field(converter=_graph_location__entity_path__special_field_converter_override)
    # The entity path that specifies where to find the node.
    #
    # (Docstring intentionally commented out to hide this field from the docs)

    node_id: datatypes.GraphNodeId = field(converter=_graph_location__node_id__special_field_converter_override)
    # The id of the node.
    #
    # (Docstring intentionally commented out to hide this field from the docs)


GraphLocationLike = GraphLocation
GraphLocationArrayLike = Union[
    GraphLocation,
    Sequence[GraphLocationLike],
]


class GraphLocationType(BaseExtensionType):
    _TYPE_NAME: str = "rerun.datatypes.GraphLocation"

    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct([
                pa.field("entity_path", pa.utf8(), nullable=False, metadata={}),
                pa.field("node_id", pa.utf8(), nullable=False, metadata={}),
            ]),
            self._TYPE_NAME,
        )


class GraphLocationBatch(BaseBatch[GraphLocationArrayLike]):
    _ARROW_TYPE = GraphLocationType()

    @staticmethod
    def _native_to_pa_array(data: GraphLocationArrayLike, data_type: pa.DataType) -> pa.Array:
        from rerun.datatypes import EntityPathBatch, GraphNodeIdBatch

        if isinstance(data, GraphLocation):
            data = [data]

        return pa.StructArray.from_arrays(
            [
                EntityPathBatch([x.entity_path for x in data]).as_arrow_array().storage,  # type: ignore[misc, arg-type]
                GraphNodeIdBatch([x.node_id for x in data]).as_arrow_array().storage,  # type: ignore[misc, arg-type]
            ],
            fields=list(data_type),
        )
