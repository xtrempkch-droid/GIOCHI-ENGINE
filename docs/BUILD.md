# Build e dependências

## Linux de desenvolvimento

Instale Rust 1.80 ou superior com `rustfmt` e `clippy`, compilador C/C++, CMake, `pkg-config` e
as bibliotecas SDL2 listadas no README. Em Debian/Ubuntu:

```sh
sudo apt-get install build-essential pkg-config cmake \
  libsdl2-dev libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev \
  libdrm-dev libgbm-dev libegl1-mesa-dev libgles2-mesa-dev libudev-dev libasound2-dev
```

O crate ativa `bundled` para SDL2 principal; imagem, mixer e TTF precisam das bibliotecas de
desenvolvimento disponíveis no sistema.

```sh
cargo build
cargo test
cargo run --bin giochi-demo
cargo run --example hello_rect
cargo run --example moving_sprite
cargo run --example input_test
```

`giochi-demo` abre uma janela preta e pode ser encerrado com Escape. `hello_rect` anima uma
primitiva, `moving_sprite` demonstra textura e movimento e `input_test` reúne teclado, mouse e
gamepad. Todos requerem um display ou uma TTY com KMSDRM.

## Verificações

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo test
cargo doc --no-deps
```

Para ARM64, consulte [CROSS_COMPILE.md](CROSS_COMPILE.md).
