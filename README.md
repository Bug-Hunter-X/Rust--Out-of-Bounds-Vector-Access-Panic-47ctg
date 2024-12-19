# Rust Out-of-Bounds Vector Access Example

This repository demonstrates a common error in Rust: panicking due to out-of-bounds vector access.  The `bug.rs` file contains code that will panic if an invalid index is used to access a vector. The `bugSolution.rs` file shows how to safely handle this situation using `get()` method.

## How to Run

1. Clone this repository.
2. Navigate to the repository's directory.
3. Run the buggy code using `rustc bug.rs && ./bug`.
4. Observe the panic.
5. Run the corrected code using `rustc bugSolution.rs && ./bugSolution`.
6. Observe that the program runs without panicking.