# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/graph_edge_undirected.fbs".

# You can extend this class by creating a "GraphEdgeUndirectedExt" class in "graph_edge_undirected_ext.py".

from __future__ import annotations

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentMixin,
)

__all__ = ["GraphEdgeUndirected", "GraphEdgeUndirectedBatch", "GraphEdgeUndirectedType"]


class GraphEdgeUndirected(datatypes.GraphEdge, ComponentMixin):
    """**Component**: An undirected edge in a graph connecting two nodes."""

    _BATCH_TYPE = None
    # You can define your own __init__ function as a member of GraphEdgeUndirectedExt in graph_edge_undirected_ext.py

    # Note: there are no fields here because GraphEdgeUndirected delegates to datatypes.GraphEdge
    pass


class GraphEdgeUndirectedType(datatypes.GraphEdgeType):
    _TYPE_NAME: str = "rerun.components.GraphEdgeUndirected"


class GraphEdgeUndirectedBatch(datatypes.GraphEdgeBatch, ComponentBatchMixin):
    _ARROW_TYPE = GraphEdgeUndirectedType()


# This is patched in late to avoid circular dependencies.
GraphEdgeUndirected._BATCH_TYPE = GraphEdgeUndirectedBatch  # type: ignore[assignment]