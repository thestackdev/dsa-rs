# dsa-rs

Data Structures and Algorithms implemented in Rust for learning purposes.

## Implementations

### Binary Search
`src/binary_search.rs`

Generic binary search on sorted slices.

```rust
pub fn binary_search<T: Ord>(array: &[T], search_key: T) -> Result<usize, usize>
```

- Returns `Ok(index)` if found
- Returns `Err(index)` with insertion point if not found

### Linked List
`src/linked_list.rs`

Singly linked list implementation.

**Methods:**
| Method | Description |
|--------|-------------|
| `new()` | Create empty list |
| `push_front(value)` | Add to beginning |
| `pop_front()` | Remove from beginning |
| `push_back(value)` | Add to end |
| `pop_back()` | Remove from end |
| `len()` | Count nodes |
| `peek()` | Reference to first value |
| `is_empty()` | Check if empty |
| `clear()` | Remove all nodes |
| `get_value(index)` | Get value at position |

## Usage

```bash
cargo build
cargo run
cargo test
```

## License

MIT
