---
title: Spawn Viewer
tags: [spawn]
c: https://github.com/rerun-io/rerun/tree/latest/examples/c/spawn_viewer/main.c
cpp: https://github.com/rerun-io/rerun/tree/latest/examples/cpp/spawn_viewer/main.cpp
rust: https://github.com/rerun-io/rerun/tree/latest/examples/rust/spawn_viewer/src/main.rs
---

Shows how to spawn a new Rerun Viewer process ready to listen for TCP connections using an executable available in PATH.

```bash
cmake .
cmake --build . --target spawn_viewer
./examples/cpp/spawn_viewer/spawn_viewer
```