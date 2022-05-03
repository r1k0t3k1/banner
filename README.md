# banner-na
welcome banner generetor

# Installation
add dependencies
```cargo.toml

[dependencies]                                                                                                                    
banner-na = { git = "https://github.com/riko-teki/banner-na.git"}
```
```
$ cargo build
```

# Usage
```rust
use banner_na;

fn main() {
  banner_na::banner("HELLO");
}

```

# Output
```
  ╗     ╗        ╗  ╗        ╗             ╗ 
  ║     ║  ╔═════╝  ║        ║        ╔═══  ╗
  ║     ║  ║        ║        ║        ║     ║
        ║       ╗   ║        ║        ║     ║
  ╔═══  ║  ╔════╝   ║        ║        ║     ║
  ║     ║  ║        ║        ║        ║     ║
  ║     ║        ╗        ╗        ╗       ╔╝
╚═╝   ╚═╝╚═══════╝╚═══════╝╚═══════╝ ╚═════╝ 

```
