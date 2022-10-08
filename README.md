# An easy interface to tracing

```rust
use tracing_ez::{span, info};

fn main() {
    tracing_ez::set_global_default_stdout();
    span!("main");
    info!("Hello, world!");
}

```