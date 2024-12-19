# RSS Reader

## Setup

Clone the repo `git clone https://github.com/gustavosvalentim/rss-reader`.

Run the CLI `cargo run -p cli`

## Parser

```rust
use rss_parser::Channel;

let content = "<channel><title>Test</title></channel>";
let channel = Channel::from(content);
```
