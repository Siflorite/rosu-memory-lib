# rosu-memory C Bindings

C bindings for the rosu-memory-lib, allowing you to read osu! memory from any language that supports C FFI.

## Building

The bindings are built using Rust's build system, Cargo. You'll need:
- Rust toolchain (1.70 or later)
- A C compiler (MSVC on Windows, gcc/clang on Linux)

```bash
cd bindings/c
cargo build --release
```

This will generate:
- A dynamic library (`rosu_memory.dll` on Windows, `librosu_memory.so` on Linux)
- A C header file (`rosu_memory.h`)

## Usage

1. Include the generated header file in your C code:
```c
#include "rosu_memory.h"
```

2. Link against the generated library when compiling:
```bash
# Windows (MSVC)
cl your_program.c rosu_memory.lib

# Linux
gcc your_program.c -lrosu_memory
```

## Example

Here's a simple example that reads and displays the current beatmap information:

```c
#include <stdio.h>
#include "rosu_memory.h"

int main() {
    State* state;
    Process* process;
    
    // Initialize the memory reader
    if (rosu_init_loop(500, &state, &process) != 0) {
        printf("Failed to initialize\n");
        return 1;
    }

    // Get beatmap info from memory
    RosuBeatmapInfo* info = rosu_memory_get_beatmap_info(process, state);
    if (info != NULL) {
        printf("Current beatmap:\n");
        printf("Title: %s\n", info->metadata.title_romanized);
        printf("Artist: %s\n", info->metadata.author);
        printf("Difficulty: %s\n", info->metadata.difficulty);
        printf("Star Rating: %.2f\n", info->stats.star_rating);
        
        // Don't forget to free the memory!
        rosu_free_beatmap_info(info);
    }

    return 0;
}
```

## Memory Management

The bindings follow C conventions for memory management:
- Functions that return pointers give you ownership of the memory
- You must free this memory using the corresponding free function
- Null pointers are returned on errors

Available free functions:
- `rosu_free_string`: Free a string returned by any function
- `rosu_free_beatmap_stats`: Free a RosuBeatmapStats structure
- `rosu_free_beatmap_info`: Free a RosuBeatmapInfo structure (also frees all contained strings)

## Error Handling

The bindings use C idioms for error handling:
- Functions returning pointers return NULL on error
- Functions returning integers return -1 on error
- Some functions return enums with an "Unknown" or error state

## Thread Safety

The library is not thread-safe by default. If you need to use it from multiple threads:
- Create separate State and Process instances for each thread
- Don't share these instances between threads
- Each thread should have its own memory reading loop

## API Reference

### Initialization
- `rosu_init_loop`: Initialize the memory reader
- `rosu_wait_for_play`: Wait until the game enters gameplay state

### Beatmap Reading
- Memory functions (current beatmap):
  - `rosu_memory_get_beatmap_md5`
  - `rosu_memory_get_beatmap_id`
  - `rosu_memory_get_beatmap_stats`
  - And more...

- File functions (read .osu files):
  - `rosu_file_get_beatmap_info`

### Memory Management
- `rosu_free_string`
- `rosu_free_beatmap_stats`
- `rosu_free_beatmap_info`

Check the header file `rosu_memory.h` for the complete API reference. 