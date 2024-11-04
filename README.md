# hpaf

High-pass audio filter in rust.

# Audio format support

- **WAV**
- **MP3** _(Experimental)_

> **_NOTE:_** **MP3** format is only partially supported and may not work reliably.

# Installation

```
git clone https://www.github.com/sebasion9/hpaf.git
cd hpaf
cargo build --release
```

# Example Usage

```
./target/release/hpaf -s example.wav -o example.out.wav -f 500
```

# Parameters
- `-s` Source audio file
- `-o` Output audio file
- `-f` Cutoff frequency (in Hz)

# Disclaimer

This project is for experimental and/or personal use only and is not intended for production. Contributions and improvements are welcome.
