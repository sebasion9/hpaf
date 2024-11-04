# hpaf

High-pass audio filter in rust.

# Audio format support

As of today, only **wav** format is supported, the project aims to also support **mp3**.

# Installation

```
git clone https://www.github.com/sebasion9/hpaf.git
```

# Example Usage

```
cargo build --release
./target/release/hpaf -s example.wav -o example.out.wav -f 500
```

