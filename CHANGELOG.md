# Registro de alterações

Formato baseado em [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/) e
[Versionamento Semântico](https://semver.org/lang/pt-BR/).

## [Não publicado]

### Corrigido

- Release publicada sem conteúdo: o workflow agora extrai a seção da versão do `CHANGELOG.md` e
  usa as notas autogeradas do GitHub apenas como fallback, evitando o "Full Changelog" duplicado.
  O nome da Release também passou a seguir a tag (`Giochi Engine vX.Y.Z`).

## [0.1.0] - 2026-11-06

Primeira versão funcional da engine, validada em hardware ARM64 com KMSDRM.

### Adicionado

- **Plataforma** (`platform`): inicialização SDL2 com seleção de KMSDRM apenas quando
  `SDL_VIDEODRIVER` não está definido, diagnóstico de permissões de `/dev/dri` e grupos
  `input`/`video`, janela com canvas, alvo de desenho (`RenderTarget`) e fábrica de texturas.
- **Loop** (`core::game_loop`): timestep fixo de 1/60 s com acumulador, limite de 250 ms por
  quadro e `FixedStepper` reutilizável; `Game` para o caminho mínimo e `run_frames` por callbacks.
- **Tempo** (`core::time`): delta, tempo decorrido, FPS, contagem de quadros e clamp de atrasos.
- **Fachada** (`core::Engine`): reúne SDL2, janela, entrada, cache de recursos, áudio opcional e
  pilha de cenas; `EngineConfig`, `EngineError`, `bootstrap`, `run`, `step`, `draw_frame` e
  `read_pixels`.
- **Cenas** (`scene`): trait `Scene` (`on_enter`, `on_exit`, `update`, `render`, `handle_event`),
  `SceneContext` e `SceneManager` com `push`/`pop`/`replace` aplicados entre quadros.
- **Input** (`input`): teclado com bordas por quadro e `movement()` (setas + WASD), mouse com
  posição/botões/roda e gamepad com botões neutros, sticks com zona morta e gatilhos normalizados.
- **Renderização** (`render`): contexto sobre SDL2 com primitivas, texturas (arquivo, bytes e RGBA
  procedural), recorte/flip/rotação, captura de pixels, `Sprite` compartilhado por `Rc` e `Camera`
  com conversão mundo/tela.
- **Assets** (`assets`): `ResourceManager` com cache de texturas, sons e músicas por `Rc` e
  estatísticas de acerto/falta, `FontCache`/`FontLoader` para SDL_ttf e erros tipados de caminho,
  leitura e decodificação.
- **Áudio** (`audio`): `AudioSystem` sobre SDL2_mixer com negociação de especificação e canais,
  `Sound` (arquivo, PCM ou tom sintetizado) e `MusicTrack` com reprodução, pausa, retomada e volume.
- **Matemática** (`math`): `Vec2` (glam), `Rect` com interseção e `Color` RGB/RGBA, sem dependência
  de SDL2.
- **Exemplos**: `hello_rect`, `moving_sprite`, `input_test`, `audio_test` e `mini_game` (Pong
  jogável com menu, placar, sons e troca de cena).
- **Feature `sdl2-system`** para compilar contra o SDL2 da imagem embarcada via `pkg-config`,
  alternativa ao `sdl2-bundled` padrão.
- **CI/CD**: matriz x86_64/ARM64 com `cross`, job de MSRV, documentação com `RUSTDOCFLAGS=-D
  warnings`, verificação de links em Markdown, `actionlint`, release por tag com binário, exemplos,
  `rlib`, tarballs e `SHA256SUMS`.
- **Documentação**: `ARCHITECTURE`, `BUILD`, `CROSS_COMPILE`, `KMSDRM`, `DECISIONS`, `API_GUIDE`,
  `PROGRESS`, `CONTRIBUTING` e templates de issue/PR.

### Corrigido

- `math::Rect::intersects` considerava retângulos sem área (largura ou altura zero) como
  colisão. Agora retângulos vazios nunca interseptam, o que evita falsos positivos em
  projéteis/áreas desativadas. A comparação continua em `i64` para não estourar perto de
  `i32::MAX`, e o novo `Rect::is_empty` torna a regra explícita.

### Validado

- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, `cargo doc` e
  `cargo check` em x86_64 e aarch64.
- Execução em KMSDRM no R35S (RK3326, Buildroot 2024.02, kernel 6.12), com gamepad `r36s_Gamepad`
  detectado e quadro conferido por captura de pixels.
