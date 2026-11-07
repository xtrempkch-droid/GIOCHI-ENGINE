# Guia da API

Como usar a `giochi-engine` para escrever um jogo 2D que roda no console Linux
(KMSDRM) e em desktop. Todos os exemplos abaixo compilam contra a API pública.

## Índice

- [Instalação](#instalação)
- [Dois caminhos de uso](#dois-caminhos-de-uso)
- [Primeiro jogo: `Engine` + `Scene`](#primeiro-jogo-engine--scene)
- [Ciclo de vida e transições de cena](#ciclo-de-vida-e-transições-de-cena)
- [Entrada](#entrada)
- [Renderização](#renderização)
- [Assets e cache](#assets-e-cache)
- [Áudio](#áudio)
- [Loop manual](#loop-manual)
- [Testes headless](#testes-headless)
- [Erros](#erros)
- [Armadilhas e boas práticas](#armadilhas-e-boas-práticas)

## Instalação

```toml
[dependencies]
giochi-engine = { git = "https://github.com/xtrempkch-droid/GIOCHI-ENGINE" }
```

Por padrão o SDL2 é compilado do código-fonte (`sdl2-bundled`). Em imagens que já
trazem SDL2 com KMSDRM (Buildroot, distribuições embarcadas), use:

```toml
[dependencies]
giochi-engine = { git = "...", default-features = false, features = ["sdl2-system"] }
```

O título curto do crate é `giochi_engine`:

```rust
use giochi_engine::core::engine::{Engine, EngineConfig};
```

## Dois caminhos de uso

| Caminho | Use quando | Ponto de entrada |
|---|---|---|
| **Completo** | Jogo com menu, fases, áudio e assets | [`Engine`] + [`Scene`] |
| **Mínimo** | Protótipo, teste de renderização, demos | [`game_loop::run`] + [`Game`] |

`Engine` é a fachada: possui SDL2, entrada, cache de recursos, áudio e a pilha de
cenas. `Game` é o loop fixo puro, sem cenas nem cache.

[`Engine`]: ../src/core/engine.rs
[`Scene`]: ../src/scene/scene.rs
[`Game`]: ../src/core/game_loop.rs
[`game_loop::run`]: ../src/core/game_loop.rs

## Primeiro jogo: `Engine` + `Scene`

```rust
use giochi_engine::{
    core::engine::{Engine, EngineConfig},
    input::Input,
    math::{color::Color, rect::Rect},
    platform::{init_sdl, Window},
    render::{renderer::RenderContext, RenderError},
    scene::Scene,
};

struct Paddle {
    y: f32,
}

impl Scene for Paddle {
    fn update(&mut self, dt: f32, input: &Input) {
        self.y = (self.y + input.keyboard.movement().y * 320.0 * dt).clamp(0.0, 400.0);
    }

    fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
        ctx.clear(Color::rgb(12, 16, 24));
        ctx.draw_rect(Rect::new(40, self.y as i32, 14, 80), Color::rgb(90, 220, 160))
    }
}

fn main() -> anyhow::Result<()> {
    let sdl = init_sdl()?;
    let config = EngineConfig::new("Meu jogo", 640, 480);
    let mut window = Window::new(&sdl, &config.title, config.width, config.height)?;
    let engine = Engine::new(sdl, &mut window, config)?;
    engine.run(Box::new(Paddle { y: 200.0 }))?;
    Ok(())
}
```

Pontos importantes:

- A **janela é emprestada** por `Engine`. Ela precisa sobreviver ao `run`; por isso
  é criada antes e passada por `&mut`.
- `Engine::run` consome a `Engine` e encerra com Quit/Escape ou quando a pilha de
  cenas fica vazia.
- Para inicializar e rodar em uma chamada, sem lidar com lifetimes, use
  `core::engine::bootstrap`:

```rust
use giochi_engine::core::engine::{bootstrap, EngineConfig};

bootstrap(EngineConfig::new("Meu jogo", 640, 480), Box::new(Paddle { y: 200.0 }))?;
```

## Ciclo de vida e transições de cena

```rust
use giochi_engine::{
    assets::ResourceManager,
    audio::AudioSystem,
    input::Input,
    render::{renderer::RenderContext, RenderError},
    scene::{Scene, SceneContext},
};

struct GameOver { score: u32 }

impl Scene for GameOver {
    fn on_enter(&mut self, ctx: &mut SceneContext<'_, '_>) {
        // `ctx.resources` é o cache da engine; `ctx.audio` é opcional.
        let _: &mut ResourceManager<'_> = ctx.resources;
        let _: Option<&mut AudioSystem> = ctx.audio;
    }

    fn on_exit(&mut self) {
        // parar música, liberar estado da fase
    }

    fn update(&mut self, _dt: f32, input: &Input) {
        if input.keyboard.is_pressed_this_frame(sdl2::keyboard::Keycode::Space) {
            self.score = 0;
        }
    }

    fn render(&mut self, ctx: &mut RenderContext<'_>) -> Result<(), RenderError> {
        ctx.clear(giochi_engine::math::color::Color::rgb(20, 0, 0));
        Ok(())
    }
}
```

### Métodos do trait

| Método | Obrigatório | Chamado quando |
|---|---|---|
| `on_enter(&mut SceneContext)` | não | a cena passa a ser o topo da pilha |
| `on_exit()` | não | a cena sai da pilha (pop, replace, push) |
| `update(dt, &Input)` | **sim** | uma vez por passo fixo de 1/60 s, só na cena do topo |
| `render(&mut RenderContext)` | **sim** | uma vez por quadro, só na cena do topo |
| `handle_event(&Event) -> bool` | não | por evento SDL, do topo para a base; `true` consome |

### Transições

```rust
engine.scenes_mut().push(Box::new(PauseMenu));   // empilha sobre a atual
engine.scenes_mut().pop();                       // volta para a anterior
engine.scenes_mut().replace(Box::new(LevelTwo)); // troca o topo
```

Transições são **agendadas e aplicadas entre quadros**, nunca no meio de
`update`/`render`. Consequências:

- A cena nova recebe `on_enter` ao final do quadro em que foi pedida e **desenha a
  partir do quadro seguinte**.
- `push` chama `on_exit` da cena anterior (ela fica abaixo, congelada).
- `pop` chama `on_exit` da removida e `on_enter` **de novo** na cena que volta ao
  topo — útil para recarregar estado.
- `replace` chama `on_exit` do topo antigo e `on_enter` do novo.

Para jogos que controlam o ritmo (e para testes), use `Engine::step` e consulte
`engine.input()`:

```rust
while engine.step(1.0 / 60.0)? {
    if engine.input().keyboard.is_pressed_this_frame(sdl2::keyboard::Keycode::Escape) {
        break;
    }
}
```

## Entrada

`Input` agrega teclado, mouse e gamepad. `is_down` é o estado contínuo;
`is_pressed_this_frame` e `is_released_this_frame` são bordas válidas só no quadro
atual.

### Teclado

```rust
let dir = input.keyboard.movement();           // setas + WASD, normalizado
input.keyboard.is_down(Keycode::Space);
input.keyboard.is_pressed_this_frame(Keycode::Return);
input.keyboard.is_released_this_frame(Keycode::Escape);
for key in input.keyboard.pressed() { /* ... */ }
```

`KeyDown` repetido do SDL2 é ignorado, então ações por borda não disparam em
cascata.

### Mouse

```rust
use giochi_engine::input::MouseButton;

let (x, y) = input.mouse.position();
if input.mouse.is_pressed_this_frame(MouseButton::Left) { /* clique */ }
let (scroll_x, scroll_y) = input.mouse.wheel();  // acumulado do quadro
```

### Gamepad

O primeiro controle é aberto automaticamente; ausência não é erro.

```rust
use giochi_engine::input::GamepadButton;

if input.gamepad.is_connected() {
    let move_dir = input.gamepad.left_stick();          // zona morta de 0.2
    let aim = input.gamepad.right_stick();
    let (lt, rt) = input.gamepad.triggers();            // 0.0 ..= 1.0
    input.gamepad.is_pressed_this_frame(GamepadButton::South);
}
```

Botões usam nomes neutros: `South`/`East`/`West`/`North` (A/B/X/Y no Xbox,
cruz/círculo/quadrado/triângulo no DualShock), `DpadUp`…`DpadRight`,
`LeftShoulder`, `Start`, `Back`, `Guide`, `LeftStick`, `RightStick`.

Ajuste a zona morta com `input.gamepad.set_deadzone(0.15)`.

## Renderização

Tudo recebe `math::Color` e `math::Rect` — nenhum tipo SDL aparece na API de jogo.

```rust
use giochi_engine::math::{color::Color, rect::Rect};

ctx.clear(Color::rgb(12, 16, 24));                       // RGBA, alpha 255
ctx.draw_rect(Rect::new(32, 48, 64, 64), Color::rgb(240, 90, 40))?;

// Textura inteira
ctx.draw_texture(&texture, Rect::new(0, 0, 64, 64))?;

// Recorte + flip + rotação
ctx.draw_texture_ex(
    &texture,
    Some(Rect::new(0, 0, 32, 32)),   // região de origem
    Rect::new(100, 100, 64, 64),     // destino em pixels
    true,                            // inverter horizontalmente
    false,                           // inverter verticalmente
    15.0,                            // ângulo em graus
)?;

let rgba = ctx.read_pixels(Rect::new(0, 0, 1, 1))?;      // captura/screenshot
```

`RenderContext::ensure_cleared(cor)` limpa apenas se a cena ainda não limpou — a
`Engine` usa isso para garantir fundo definido sem `clear` duplicado.

### Sprites

`Sprite` compartilha a textura por `Rc` e desenha uma região:

```rust
use std::rc::Rc;
use giochi_engine::render::sprite::Sprite;

let texture = engine.texture("assets/hero.png")?;        // Rc<Texture>
let mut hero = Sprite::new(Rc::clone(&texture));
hero.src_rect = Some(Rect::new(0, 0, 48, 48));           // frame do atlas

hero.draw(ctx, Rect::new(200, 120, 96, 96))?;
```

### Câmera

`Camera` converte mundo → tela com posição central e zoom:

```rust
use giochi_engine::render::camera::Camera;
use giochi_engine::math::{rect::Rect, vec2::Vec2};

let camera = Camera {
    position: Vec2::new(640.0, 360.0),                   // centro no mundo
    zoom: 2.0,
    viewport: Rect::new(0, 0, 1280, 720),
};

let screen = camera.world_to_screen(Vec2::new(660.0, 380.0)); // (680, 400)
let dest = camera.apply(Rect::new(640, 360, 32, 32));         // (640, 360, 64, 64)
ctx.draw_texture(&tile, dest)?;
```

### Texturas procedurais

Úteis para protótipos e para imagens sem arquivo:

```rust
let loader = engine.texture_loader()?;
let pixels: Vec<u8> = vec![255, 0, 0, 255].repeat(32 * 32);   // RGBA8
let texture = Texture::from_rgba(&loader, 32, 32, &pixels)?;
```

O buffer precisa ter exatamente `largura * altura * 4` bytes; caso contrário a
chamada falha com `RenderError::InvalidImageData`.

## Assets e cache

`ResourceManager` carrega cada recurso uma vez e compartilha por `Rc`:

```rust
let texture = engine.texture("assets/tiles.png")?;            // atalho
let sound = engine.resources_mut().sound("assets/hit.wav")?;
let music = engine.resources_mut().music("assets/theme.ogg")?;

let stats = engine.resources().stats();
println!("{} cargas para {} pedidos", stats.loads(), stats.requests());
```

Duas chamadas para o mesmo caminho produzem `loads() == 1` e `Rc::ptr_eq` — é o
critério coberto pelo teste de integração.

`CacheStats` expõe `hits`, `misses`, `loads` e `requests`; `ResourceManager::clear`
esvazia tudo (útil ao trocar de fase com muitos assets).

### Fontes

Fontes têm lifecycle próprio (`FontCache`), porque dependem do `Sdl2TtfContext`:

```rust
use giochi_engine::assets::{FontCache, FontLoader};

let loader = FontLoader::with_base_dir("assets/fonts")?;
let mut fonts = FontCache::new();
let font = fonts.font(&loader, "ui.ttf", 16)?;
```

**Ordem de declaração importa.** `FontCache` empresta o `FontLoader`, e o Rust libera
variáveis na ordem inversa da declaração: declare o loader **antes** do cache. O
mesmo vale para `ResourceManager` e a janela/loader de texturas.

### Erros de asset

`AssetError` distingue `InvalidPath` (não existe / não é arquivo), `Io` (falha de
leitura) e `Texture`/`Audio`/`Font` (falha de decodificação). Trate recurso
ausente sem derrubar o jogo:

```rust
match engine.texture("assets/opcional.png") {
    Ok(texture) => { /* usa */ }
    Err(error) => log::warn!("textura opcional ausente: {error}"),
}
```

## Áudio

O áudio é **opcional**: se o dispositivo não abrir, `engine.audio()` é `None` e o
jogo segue em silêncio.

```rust
if let Some(audio) = engine.audio() {
    println!("{} Hz, {} canais", audio.spec().frequency, audio.spec().channels);
}
```

### Efeitos

```rust
use giochi_engine::audio::Sound;

let beep = Sound::tone(880.0, 0.15, 44_100)?;   // síntese, sem arquivo
beep.play()?;                                    // qualquer canal livre
beep.play_loops(-1)?;                            // repetir indefinidamente
```

`Sound::from_file` carrega WAV/OGG conforme a build do SDL2_mixer;
`Sound::from_samples(&[i16])` aceita PCM 16 bits já no formato do dispositivo
(use `audio.spec()` para a taxa e os canais corretos).

### Música

```rust
if let Some(music) = engine.resources_mut().music("assets/theme.ogg").ok() {
    music.play_loop()?;
    music.pause();      // pausa/retomada são globais no SDL_mixer
    music.resume();
    music.stop();
}
```

O controle de música é global: existe uma música por vez na `AudioSystem`.

### Inicialização manual

Fora da `Engine`:

```rust
use giochi_engine::audio::{AudioSystem, MixerConfig};

let mut audio = AudioSystem::init(MixerConfig::MONO)?;   // ou ::with_defaults()
audio.allocate_channels(8);                              // só cresce, nunca reduz
```

## Loop manual

Para jogos sem cenas, o loop fixo aceita callbacks independentes:

```rust
use giochi_engine::core::game_loop::{run_frames, FIXED_DT};

run_frames(
    || { /* true para continuar */ true },
    |dt| { /* atualização de passo fixo */ let _ = dt; },
    || { /* desenho do quadro */ Ok(()) },
)?;
```

Ou implemente `Game` e use `game_loop::run(&mut game, &mut window, &mut sdl)`, que
cuida de eventos, entrada e `present`.

`FixedStepper` é a peça reutilizável do loop (`advance()` devolve quantos passos
rodar; `wait()` mantém 60 Hz) para quem precisa de um loop próprio.

## Testes headless

O driver `dummy` do SDL2 permite testar renderização e entrada sem display:

```rust
std::env::set_var("SDL_VIDEODRIVER", "dummy");

let sdl = giochi_engine::platform::init_sdl()?;
let mut window = giochi_engine::platform::Window::new(&sdl, "teste", 64, 48)?;
let mut engine = Engine::new(sdl, &mut window, EngineConfig::new("teste", 64, 48).with_audio(false))?;

engine.scenes_mut().push(Box::new(MyScene));
engine.step(1.0 / 60.0)?;                       // on_enter + update + render
let px = engine.read_pixels(Rect::new(0, 0, 1, 1))?;
assert_eq!(&px[..3], &[12, 16, 24]);
```

O SDL2 permite **um** contexto por processo: serialize testes que o inicializam com
um `Mutex` estático (veja `tests/integration.rs`). Para áudio em CI, use
`SDL_AUDIODRIVER=dummy`.

## Erros

| Tipo | Camada | Uso |
|---|---|---|
| `RenderError` | renderização | falha de desenho, textura, dados RGBA inválidos |
| `AssetError` | assets | caminho, I/O, decodificação |
| `AudioError` | áudio | inicialização, dispositivo, carga, reprodução |
| `PlatformError` | plataforma | SDL2, vídeo, janela, eventos |
| `EngineError` | fachada | agrega os anteriores com `#[from]` |

A biblioteca usa `thiserror`; binários e exemplos compõem com `anyhow`. Erros de
desenho são propagados de `Scene::render`, permitindo abortar o loop com contexto.

## Armadilhas e boas práticas

- **Janela e lifetimes.** `Engine<'w>` empresta a janela; o cache de texturas
  contém `Texture<'w>` ligadas ao criador SDL dela. Não tente mover a `Engine` para
  outra thread nem liberar a janela antes do fim do `run`.
- **Ler a janela do jogo.** Não acesse o canvas diretamente: use
  `RenderContext`. Ele é o único ponto que traduz `Color`/`Rect` para SDL2.
- **Transições entre quadros.** Se precisar que a nova cena desenhe no mesmo
  quadro, aplique a troca antes de chamar `draw_frame` (fluxo com `step`).
- **Custo do clear.** `clear` é barato, mas em cenas com fundo próprio prefira
  `ensure_cleared` (a `Engine` já faz isso) para evitar dois clears.
- **Assets em KMSDRM.** Imagens da imagem embarcada podem não ter decodificador
  PNG; teste carregamento no hardware. O SDL_image é inicializado sob demanda, então
  a ausência de um formato não impede a engine de subir.
- **Áudio embarcado.** Muitas imagens Buildroot não trazem backend ALSA. Trate
  `audio() == None` como caso normal.
- **Nomes.** API em inglês, documentação e mensagens em português brasileiro.
