# rosu-memory-lib

A modular Rust library for reading osu! memory, forked from [rosu_memory](https://github.com/486c/rosu_memory).

## Overview

While rosu_memory was originally designed as a binary application, this fork transforms it into a flexible library that can be integrated into any Rust project. This architectural change allows developers to build their own tools and applications around osu! memory reading capabilities.

## Features

- Memory reading for stable (lazer support may come soon)
- Modular design allowing custom implementations
- Callback system for real-time data processing
- Type-safe memory access
- Cross-platform support (Windows primary focus)


Basic example:
```rust
use rosu_memory_lib::reader::{init_loop, waiting_for_play};

fn main() -> eyre::Result<()> {
    // Initialize the memory reader
    let (mut state, process) = init_loop(500)?;

    // Wait for gameplay with optional callback
    waiting_for_play(&process, &mut state, Some(|p, state| {
        println!("beatmap title: {}", rosu_memory_lib::reader::beatmap::stable::get_title(p,state))
        Ok(())
    }))?;

    Ok(())
}
```

## Examples

Check out the `examples/` directory for various implementations and use cases.

## Language Bindings

The library provides bindings for multiple programming languages (not finished):

### Python
Python bindings are available in the `bindings/python` directory. They provide a Pythonic interface to the library's functionality using PyO3.

Example:
```python
from rosu_memory import init_loop, get_beatmap_info

# Initialize the memory reader
state, process = init_loop(500)

# Get current beatmap info
beatmap = get_beatmap_info(process, state)
print(f"Playing: {beatmap.metadata.title_romanized}")
```

### C
C bindings are available in the `bindings/c` directory. They provide a C-compatible interface that can be used from any language supporting C FFI.

Example:
```c
#include "rosu_memory.h"

int main() {
    State* state;
    Process* process;
    rosu_init_loop(500, &state, &process);

    RosuBeatmapInfo* info = rosu_memory_get_beatmap_info(process, state);
    if (info != NULL) {
        printf("Playing: %s\n", info->metadata.title_romanized);
        rosu_free_beatmap_info(info);
    }

    return 0;
}
```

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## Credits

- Original [rosu_memory](https://github.com/486c/rosu-memory) by 486c
- All contributors to the original project

## License

Same as original rosu_memory
