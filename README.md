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

Compiladores cruzados (apenas para ARM64 local): `gcc-aarch64-linux-gnu` e, para a
feature `sdl2-system`, um sysroot com as bibliotecas SDL do dispositivo — veja
[docs/CROSS_COMPILE.md](docs/CROSS_COMPILE.md).

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
a cor e o botão direito reposiciona o quadrado sob o ponteiro. `cargo run --example audio_test`
sintetiza dois tons (Espaço e Enter) usando `AudioSystem` e `Sound`, sem arquivos de áudio.
`cargo run --example mini_game` abre um Pong jogável (W/S ou setas movem, Espaço inicia) que usa
todos os módulos — entrada, renderização, assets, áudio, cenas e matemática.

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

Para imagens embarcadas que já entregam SDL2 com KMSDRM, compile contra o SDL2 do próprio
aparelho em vez de recompilá-lo:

```sh
PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH=/tmp/r35s-sysroot/usr/lib/pkgconfig \
cargo build --release --target aarch64-unknown-linux-gnu \
      --no-default-features --features sdl2-system
```

A feature `sdl2-bundled` (padrão) compila o SDL2 do código-fonte; `sdl2-system` usa o SDL2 do
sistema via `pkg-config`. O arquivo [docs/CROSS_COMPILE.md](docs/CROSS_COMPILE.md) detalha o
sysroot do dispositivo, a transferência e as dependências de runtime.

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

## Uso da engine

O caminho completo usa a fachada `Engine` com cenas:

```rust
use giochi_engine::{
    core::engine::{Engine, EngineConfig},
    input::Input,
    math::color::Color,
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
    scene::Scene,
};

struct Pong;

impl Scene for Pong {
    fn update(&mut self, dt: f32, input: &Input) {
        let _ = (dt, input.keyboard.movement());
    }

    fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
        ctx.clear(Color::rgb(12, 16, 24));
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let sdl = init_sdl()?;
    let config = EngineConfig::new("Meu jogo", 640, 480);
    let mut window = Window::new(&sdl, &config.title, config.width, config.height)?;
    let engine = Engine::new(sdl, &mut window, config)?;
    engine.run(Box::new(Pong))?;
    Ok(())
}
```

`Engine` reúne SDL2, janela, entrada, cache de recursos (`Engine::texture`), áudio opcional e a
pilha de cenas. Para jogos que não precisam de cenas, `core::game_loop::run` executa diretamente uma
implementação de `core::game_loop::Game`.

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

## Assets e áudio

`assets::ResourceManager` carrega cada recurso uma vez e compartilha por `Rc`:

```rust
use giochi_engine::assets::ResourceManager;

let (mut context, texture_loader) = window.render_context_with_loader()?;
let mut resources = ResourceManager::new();
let player = resources.texture(&texture_loader, "assets/player.png")?;
println!("cargas: {}", resources.stats().loads());
```

`audio::AudioSystem` abre o dispositivo e `Sound`/`MusicTrack` reproduzem efeitos e músicas:

```rust
use giochi_engine::audio::{AudioSystem, MixerConfig, Sound};

let mut audio = AudioSystem::init(MixerConfig::MONO)?;
audio.allocate_channels(8);
let beep = Sound::tone(880.0, 0.2, audio.spec().frequency)?;
beep.play()?;
```

## Status da engine

| Camada | Estado |
|---|---|
| `platform` (SDL2/KMSDRM, janela) | implementada |
| `core` (loop fixo, tempo, fachada `Engine`) | implementada |
| `input` (teclado, mouse, gamepad) | implementada |
| `render` (primitivas, texturas, sprites, câmera) | implementada |
| `assets` (cache, fontes) | implementada |
| `audio` (mistura, efeitos, música) | implementada |
| `scene` (trait, contexto, pilha) | implementada |

O que ainda não existe: renderização de texto na tela, física/colisão além de `Rect`,
animação de sprites por quadros e empacotamento para distribuições.

## Validação em hardware

A engine foi executada em um GIOCHI KONSOL OS (RK3326, Buildroot, kernel 6.12) a partir de um
console sem X11/Wayland, com o driver `KMSDRM` ativo e o gamepad `r36s_Gamepad` detectado. A
captura de pixels confirmou cor e geometria do quadro renderizado nos exemplos `hello_rect`,
`moving_sprite` e `input_test`. Detalhes e particularidades da imagem estão em
[docs/KMSDRM.md](docs/KMSDRM.md).

## Documentação e contribuição

- [Guia da API](docs/API_GUIDE.md) — como usar `Engine`, `Scene`, entrada, render, assets e áudio
- [Arquitetura](docs/ARCHITECTURE.md)
- [Build e dependências](docs/BUILD.md)
- [KMSDRM](docs/KMSDRM.md)
- [Cross-compilation ARM64](docs/CROSS_COMPILE.md)
- [Decisões de arquitetura](docs/DECISIONS.md)
- [Progresso da implementação](PROGRESS.md)
- [Registro de alterações](CHANGELOG.md)

Para propor correções ou funcionalidades, consulte [CONTRIBUTING.md](CONTRIBUTING.md).
