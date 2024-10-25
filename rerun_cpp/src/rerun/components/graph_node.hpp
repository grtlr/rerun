// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/store/re_types/definitions/rerun/components/graph_node.fbs".

#pragma once

#include "../datatypes/graph_node.hpp"
#include "../result.hpp"

#include <cstdint>
#include <memory>
#include <string>
#include <utility>

namespace rerun::components {
    /// **Component**: A string-based ID representing a node in a graph.
    struct GraphNode {
        rerun::datatypes::GraphNode id;

      public:
        GraphNode() = default;

        GraphNode(rerun::datatypes::GraphNode id_) : id(std::move(id_)) {}

        GraphNode& operator=(rerun::datatypes::GraphNode id_) {
            id = std::move(id_);
            return *this;
        }

        GraphNode(std::string id_) : id(std::move(id_)) {}

        GraphNode& operator=(std::string id_) {
            id = std::move(id_);
            return *this;
        }

        /// Cast to the underlying GraphNode datatype
        operator rerun::datatypes::GraphNode() const {
            return id;
        }
    };
} // namespace rerun::components

namespace rerun {
    static_assert(sizeof(rerun::datatypes::GraphNode) == sizeof(components::GraphNode));

    /// \private
    template <>
    struct Loggable<components::GraphNode> {
        static constexpr const char Name[] = "rerun.components.GraphNode";

        /// Returns the arrow data type this type corresponds to.
        static const std::shared_ptr<arrow::DataType>& arrow_datatype() {
            return Loggable<rerun::datatypes::GraphNode>::arrow_datatype();
        }

        /// Serializes an array of `rerun::components::GraphNode` into an arrow array.
        static Result<std::shared_ptr<arrow::Array>> to_arrow(
            const components::GraphNode* instances, size_t num_instances
        ) {
            if (num_instances == 0) {
                return Loggable<rerun::datatypes::GraphNode>::to_arrow(nullptr, 0);
            } else if (instances == nullptr) {
                return rerun::Error(
                    ErrorCode::UnexpectedNullArgument,
                    "Passed array instances is null when num_elements> 0."
                );
            } else {
                return Loggable<rerun::datatypes::GraphNode>::to_arrow(
                    &instances->id,
                    num_instances
                );
            }
        }
    };
} // namespace rerun