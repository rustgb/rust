// Tests that empty source_maps don't ICE (#23301)

// compile-flags: --cfg ""

// error-pattern: invalid `--cfg` argument: `""` (expected `key` or `key="value"`, ensure escaping is appropriate for your shell, try 'key="value"' or key=\"value\")

pub fn main() {
}
