# GIOCHI-ENGINE

[![CI](https://github.com/xtrempkch-droid/GIOCHI-ENGINE/actions/workflows/ci.yml/badge.svg)](https://github.com/xtrempkch-droid/GIOCHI-ENGINE/actions/workflows/ci.yml)
[![Licença MIT](https://img.shields.io/badge/licen%C3%A7a-MIT-blue.svg)](LICENSE)
[![Rust 1.80+](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![ARM64](https://img.shields.io/badge/target-aarch64-green.svg)](docs/CROSS_COMPILE.md)

Engine 2D em Rust baseada em SDL2, com KMSDRM para execução direta em Linux embarcado.
O projeto integra o ecossistema [GIOCHI-SYSTEM](https://github.com/xtrempkch-droid/GIOCHI-SYSTEM)
e pode ser usado como biblioteca Rust ou como binário independente. O alvo primário é
`aarch64-unknown-linux-gnu`; `x86_64-unknown-linux-gnu` é suportado para desenvolvimento.

## Pré-requisitos de sistema

Em Debian/Ubuntu, instale os pacotes abaixo para desenvolvimento e execução:

| Pacote | Finalidade |
|---|---|
| `libsdl2-dev` | SDL2 e backend de vídeo |
| `libdrm-dev` | Interface DRM/KMS |
| `libgbm-dev` | Alocação de buffers gráficos |
| `libegl1-mesa-dev` | EGL |
| `libgles2-mesa-dev` | OpenGL ES |
| `libsdl2-image-dev` | Carregamento de imagens |
| `libsdl2-mixer-dev` | Áudio e mixer |
| `libsdl2-ttf-dev` | Fontes TrueType |
| `libudev-dev` | Descoberta de dispositivos |
| `libasound2-dev` | Backend ALSA |
| `pkg-config` | Descoberta de bibliotecas nativas |
| `cmake` | Construção do SDL2 bundled |
| `gcc` | Compilador C usado pelo SDL2 bundled |

```sh
sudo apt-get update
sudo apt-get install build-essential pkg-config cmake \
  libsdl2-dev libdrm-dev libgbm-dev libegl1-mesa-dev libgles2-mesa-dev \
  libsdl2-image-dev libsdl2-mixer-dev libsdl2-ttf-dev libudev-dev libasound2-dev
```

Também é necessário Rust stable com `rustfmt` e `clippy`; `rust-toolchain.toml` instala
automaticamente os componentes e o target ARM64.

## Build nativo (x86_64)

```sh
rustup target add x86_64-unknown-linux-gnu
cargo build
cargo test
cargo run --bin giochi-demo
```

O binário `giochi-demo` abre uma janela preta que pode ser encerrada com Escape. Para as
demonstrações de desenho e movimento, use `cargo run --example hello_rect` e
`cargo run --example moving_sprite`; esta última cria sua textura em memória. O exemplo
`cargo run --example input_test` exercita teclado, mouse e gamepad: setas/WASD movem, Espaço troca
a cor e o botão direito reposiciona o quadrado sob o ponteiro.

Para validar a qualidade:

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo doc --no-deps
```

## Cross-compile ARM64

Instale Docker e `cross`, e compile com:

```sh
rustup target add aarch64-unknown-linux-gnu
cargo install cross --locked
cross build --target aarch64-unknown-linux-gnu --release
```

O arquivo [docs/CROSS_COMPILE.md](docs/CROSS_COMPILE.md) detalha a cadeia de compilação,
transferência e dependências de runtime.

## Execução em KMSDRM

Execute a partir de uma TTY local (não de uma sessão gráfica Wayland/X11), com acesso ao
dispositivo DRM. O SDL2 seleciona KMSDRM por padrão quando `SDL_VIDEODRIVER` não está definido;
configure explicitamente quando necessário:

```sh
export SDL_VIDEODRIVER=KMSDRM
cargo run --release --bin giochi-demo
```

Adicione o usuário aos grupos `input` e `video` e faça novo login:

```sh
sudo usermod -aG input,video "$USER"
```

Confira [docs/KMSDRM.md](docs/KMSDRM.md) antes de executar em uma tela embarcada.

## Uso como biblioteca

Adicione a dependência no `Cargo.toml` do consumidor:

```toml
[dependencies]
giochi-engine = { git = "https://github.com/xtrempkch-droid/GIOCHI-ENGINE" }
```

Exemplo mínimo:

```rust
use giochi_engine::math::{rect::Rect, vec2::Vec2};

fn main() {
    let position = Vec2::new(32.0, 48.0);
    let bounds = Rect { x: 0, y: 0, width: 128, height: 128 };
    assert!(bounds.intersects(Rect {
        x: position.x as i32,
        y: position.y as i32,
        width: 16,
        height: 16,
    }));
}
```

## Documentação e contribuição

- [Arquitetura](docs/ARCHITECTURE.md)
- [Build e dependências](docs/BUILD.md)
- [KMSDRM](docs/KMSDRM.md)
- [Cross-compilation ARM64](docs/CROSS_COMPILE.md)
- [Decisões de arquitetura](docs/DECISIONS.md)

Para propor correções ou funcionalidades, consulte [CONTRIBUTING.md](CONTRIBUTING.md).
