# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/graph_node.fbs".

# You can extend this class by creating a "GraphNodeExt" class in "graph_node_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["GraphNode", "GraphNodeBatch", "GraphNodeType"]


class GraphNode(datatypes.Utf8, ComponentMixin):
    """**Component**: A string-based ID representing a node in a graph."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of GraphNodeExt in graph_node_ext.py

    # Note: there are no fields here because GraphNode delegates to datatypes.Utf8
    pass


class GraphNodeType(datatypes.Utf8Type):
    _TYPE_NAME: str = "rerun.components.GraphNode"


class GraphNodeBatch(datatypes.Utf8Batch, ComponentBatchMixin):
    _ARROW_TYPE = GraphNodeType()


# This is patched in late to avoid circular dependencies.
GraphNode._BATCH_TYPE = GraphNodeBatch  # type: ignore[assignment]
