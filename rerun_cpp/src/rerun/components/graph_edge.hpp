// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/graph_edge.fbs".

#pragma once

#include "../datatypes/graph_edge.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <utility>

namespace rerun::components {
    /// **Component**: An edge in a graph connecting two nodes.
    struct GraphEdge {
        rerun::datatypes::GraphEdge edge;

      public:
        GraphEdge() = default;

        GraphEdge(rerun::datatypes::GraphEdge edge_) : edge(std::move(edge_)) {}

        GraphEdge& operator=(rerun::datatypes::GraphEdge edge_) {
            edge = std::move(edge_);
            return *this;
        }

        /// Cast to the underlying GraphEdge datatype
        operator rerun::datatypes::GraphEdge() const {
            return edge;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::GraphEdge) == sizeof(components::GraphEdge));

    /// \private
    template <>
    struct Loggable<components::GraphEdge> {
        static constexpr const char Name[] = "rerun.components.GraphEdge";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::GraphEdge>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::GraphEdge` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::GraphEdge* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::GraphEdge>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::GraphEdge>::to_arrow(
                    &instances->edge,
                    num_instances
                );
            }
        }
    };
} // namespace rerun