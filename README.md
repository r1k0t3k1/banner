# banner
welcome banner generetor

# Installation
add dependencies
```cargo.toml

[dependencies]                                                                                                                    
banner-na = { git = "https://github.com/riko-teki/banner.git"}
```
```
$ cargo build
```

# Usage
```rust
use banner;

fn main() {
  banner::banner("HELLO");
}

```

# Output
![](https://raw.githubusercontent.com/riko-teki/images/main/%E3%82%B9%E3%82%AF%E3%83%AA%E3%83%BC%E3%83%B3%E3%82%B7%E3%83%A7%E3%83%83%E3%83%88_2022-05-03_20-29-59.png)
