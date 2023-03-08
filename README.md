# Transsub
Transsub is a user friendly tool to translate subtitles.

## Installation
For now we don't publish any release files. You can install it from source code. First clone the repository:
```bash
git clone https://github.com/amirheidarikhoram/transsub.git
```

Then build the app in release mode with cargo:
```bash
cargo build --release
```

You can use the build which is located at `target/release/transsub` or copy it to your bin directory:
```bash
cp target/release/transsub /usr/local/bin
```

## Usage
With the command below you can translate subtitles inside a folder from `SOURCE` language to `TARGET` language. `DIR` is the directory which contains the subtitles. Transsub will search the directory `recursively` for all files with `.srt` extension and translate them. The translated files will be saved in the same location with the original `.srt` file, with the same name but ending with`-<TARGET>.srt`. For example if you have a file named `movie.srt` in the directory, the translated file will be named `movie-<TARGET>.srt`.
```bash
transsub <SOURCE> <TARGET> -d <DIR>
```

You can store all translated files in a single directory by using the `-o` option:
```bash
transsub <SOURCE> <TARGET> -d <DIR> -o <OUT>
```

## Contribution
Any contribution is welcome. Please feel free to open an issue or a pull request.