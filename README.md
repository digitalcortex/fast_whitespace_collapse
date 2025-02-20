# fast_whitespace_collapse

A high-performance Rust crate for collapsing consecutive spaces and tabs into a single space.  
Uses **SIMD (`u8x16`) via the [`wide` crate](https://crates.io/crates/wide)** for efficient processing.  
Automatically falls back to a **scalar implementation** if SIMD is unavailable.

## Features
- Collapses multiple spaces and tabs into a single space.
- Preserves newlines and non-whitespace characters.
- Uses **SIMD (`u8x16`) when supported** to process 16 bytes at a time.
- Falls back to **a fast scalar implementation** if SIMD is unavailable.
- Ensures valid UTF-8 output.
- SIMD requires **AVX2, SSE2, or NEON** instruction sets.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fast_whitespace_collapse = "0.1"
```

Or run the following command:

```bash
cargo add fast_whitespace_collapse
```

### **Controlling SIMD Support**
By default, SIMD acceleration is **enabled**. You can control it via Cargo features:

#### **🔹 Disable SIMD for Embedded Targets**
```sh
cargo build --no-default-features
```

#### **🔹 Explicitly Enable SIMD**
```sh
cargo build --features simd-optimized
```

## Usage

```rust
use fast_whitespace_collapse::collapse_whitespace;

let input = "This   is \t  a   test.";
let output = collapse_whitespace(input);
assert_eq!(output, "This is a test.");
```

## Performance
- Processes text using **SIMD (`u8x16`)**, handling **16 bytes in parallel**.
- Falls back to **scalar processing** when SIMD is unavailable.
- Handles **large inputs efficiently** while maintaining valid UTF-8 output.

## Benchmark Results

### **Comparison with Other Approaches**

| Method | Time |
|--------|------|
| Regex approach | 11.289 µs |
| [collapse](https://crates.io/crates/collapse) crate | 1.2624 µs |
| Iterative approach | 629.60 ns |
| Iterative bytes | 428.00 ns |
| [fast_whitespace_collapse](https://crates.io/crates/fast_whitespace_collapse) crate | **388.73 ns** |

🚀 **`fast_whitespace_collapse` outperforms other methods, achieving the lowest execution time.**

📌 **Benchmark executed on Apple M1 Pro (NEON SIMD enabled).**

### **🔹 Run Your Own Benchmark**
```sh
cargo bench
```

## Compatibility

**`fast_whitespace_collapse`** supports multiple architectures:

- **x86_64**: Uses SIMD (`SSE2`, `AVX2`) for maximum performance.
- **ARM (aarch64, M1/M2/M3)**: Uses **NEON SIMD**.
- **Other**: Falls back to **a scalar implementation**.

## Examples

### **Basic Usage**
```rust
use fast_whitespace_collapse::collapse_whitespace;

assert_eq!(collapse_whitespace("Hello    world"), "Hello world");
assert_eq!(collapse_whitespace("   Trim   spaces   " ), "Trim spaces");
assert_eq!(collapse_whitespace("Tabs\t\tconverted"), "Tabs converted");
```

### **Unicode Support**
```rust
assert_eq!(collapse_whitespace("こんにちは  世界"), "こんにちは 世界"); // Japanese
assert_eq!(collapse_whitespace("你好  世界"), "你好 世界"); // Chinese
assert_eq!(collapse_whitespace("😀  😃  😄"), "😀 😃 😄"); // Emojis
```

### **Handling Newlines**
```rust
assert_eq!(collapse_whitespace("Line1\n   Line2\nLine3"), "Line1\n Line2\nLine3");
```

## Tests
Run tests with:
```sh
cargo test
```

## License
This project is licensed under the **MIT License**.