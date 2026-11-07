# Progresso da implementação

## Última fase concluída: Fase 6 — Polimento, CI e release

Data: 2026-11-06

## Fases

| Fase | Estado | Resumo |
|---|---|---|
| Fase 0 — Bootstrap | Concluída | Pacote Cargo, binário de smoke test, módulos públicos, toolchain, Cross, documentação, workflows e testes iniciais. |
| Fase 1 — Platform e loop | Concluída | Inicialização SDL2 e vídeo, wrapper de janela, tempo delta/elapsed/fps e loop fixo com saída por Quit/Escape. |
| Fase 2 — Renderização 2D | Concluída | Primitivas, texturas SDL_image/RGBA, sprites, câmera e exemplos funcionais com contexto próprio de renderização. |
| Fase 3 — Input | Concluída | Teclado, mouse e gamepad com estados por quadro, integração no loop e exemplo `input_test`. |
| Fase 4 — Assets e áudio | Concluída | Cache de texturas/sons/músicas/fontes, `AudioSystem`, efeitos, música e exemplo `audio_test`. |
| Fase 5 — Cenas e integração | Concluída | `Scene`/`SceneManager`, fachada `Engine` com todos os subsistemas e `mini_game` jogável. |
| Fase 6 — Polimento e release | Concluída | `API_GUIDE`, CI com docs/links/actionlint, release por tag com artefatos e changelog consolidado. |

## Gates validados na Fase 6

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo build --locked --target x86_64-unknown-linux-gnu --all-targets`
- [x] `cargo test --locked --target x86_64-unknown-linux-gnu` (34 testes: 21 unitários e 13 de
  integração; ciclo de vida de cena, transições push/pop/replace, consumo de eventos, ciclo da
  `Engine` e encerramento por Escape, além dos testes anteriores)
- [x] `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --document-private-items`
- [x] `actionlint 1.7.7` em `ci.yml`, `release.yml` e `cross-compile.yml` (inclui shellcheck dos
  scripts embutidos)
- [x] Verificação de links relativos em `README.md` e `docs/*.md`
- [x] Exemplos do `docs/API_GUIDE.md` compilados contra a API pública (`cargo check` em crate
  separado), confirmando que o guia corresponde ao código
- [x] `cross build --locked --release --target aarch64-unknown-linux-gnu --all-targets` — gate
  ARM64 executado com QEMU/binfmt local; toda a árvore (lib, binário, exemplos e testes) compila.
  `cargo check --target aarch64` nativo com a feature `bundled` não serve como gate: o CMake do
  `sdl2-sys` encontra bibliotecas x86_64 do host e o link cruzado falha (`cannot find
  -lsamplerate`). `cross check` em perfil debug esbarra em `/usr/bin/ccache` ausente na imagem;
  o gate equivalente e confiável é o `cross build --release`.
- [x] `cargo doc --locked --no-deps --document-private-items`
- [x] Execução em hardware ARM64 real (R35S/RK3326, GIOCHI KONSOL OS, Buildroot 2024.02,
  kernel 6.12): driver `KMSDRM` ativo, `/dev/dri/card0` aberto, gamepad `r36s_Gamepad` detectado e
  os exemplos `hello_rect`, `moving_sprite` e `input_test` ativos em console sem X11/Wayland.

## Entregue na Fase 6

- `docs/API_GUIDE.md`: guia completo com instalação, dois caminhos de uso, primeiro jogo,
  ciclo de vida/transições de cena, entrada, renderização, assets, áudio, loop manual, testes
  headless, tabela de erros e armadilhas (incluindo ordem de declaração de `FontCache`).
- `release.yml` reescrito: `workflow_dispatch` com `dry_run` para pré-visualizar sem publicar,
  build `--locked --all-targets`, empacotamento do binário e dos 5 exemplos, `rlib` da biblioteca,
  tarball agregado, `SHA256SUMS`, verificação de arquitetura via `file` e upload de artefato.
- `ci.yml`: build de release com `--all-targets`, `cargo doc` da feature `sdl2-system`, job de
  documentação com `RUSTDOCFLAGS=-D warnings`, verificação de links em Markdown e job `actionlint`.
- `Cargo.toml`: `readme`, `keywords` e `categories` para publicação.
- `CHANGELOG.md` consolidado em `0.1.0` (nenhuma tag existia) com seções Adicionado/Validado.
- `README.md` finalizado: tabela de status por camada, compiladores cruzados, link para o guia de
  API e para `PROGRESS`/`CHANGELOG`.
- `CONTRIBUTING.md` já cobria fmt/clippy/test/doc/actionlint e a nota de hardware ARM64.
- `packaging/LEIA-ME.txt`: requisitos de runtime e modo de execução em KMSDRM, copiado para o
  tarball do release.
- `docs/CROSS_COMPILE.md`: nova seção sobre o que o `cross` produz, a lista real de dependências do
  SDL2 `bundled` e as duas opções corretas para alvo embarcado.
- `docs/DECISIONS.md`: decisão de não distribuir o SDL2 `bundled` e registro da tentativa
  descartada (`SDL2_TOOLCHAIN`), com o motivo técnico.

## Entregue na Fase 5

- `scene::Scene` com `on_enter(SceneContext)`, `on_exit`, `update(dt, &Input)`, `render(ctx)` e
  `handle_event` opcional; o trait é object-safe e aceita `Box<dyn Scene + 'w>`.
- `scene::SceneContext<'s, 'w>` entrega `&mut ResourceManager<'w>` e áudio opcional na entrada/saída
  da cena.
- `scene::SceneManager<'w>` com `push`, `pop`, `replace`, `current`, `current_mut`, `depth`,
  `is_empty`, `handle_event` (do topo para a base) e aplicação de transições entre quadros, com
  `on_exit`/`on_enter` na ordem correta.
- `core::Engine`: reúne `Sdl`, entrada, `ResourceManager<'w>`, `RenderTarget<'w>`, `AudioSystem`
  opcional e `SceneManager<'w>`; oferece `new`, `run` (loop fixo), `step` (quadro controlado),
  `poll_frame`, `draw_frame`, `texture`, `texture_loader`, `resources`, `scenes` e `audio`.
- `core::EngineConfig` (título, dimensões, áudio, mixer), `core::EngineError` (thiserror) e
  `core::bootstrap(config, scene)` para inicializar e executar em uma chamada.
- `platform::Window::split`/`RenderTarget` permitem alvo e cache coexistirem sem `unsafe`;
  `RenderContext::ensure_cleared` garante fundo definido sem `clear` duplicado.
- `Texture` passou a ter o lifetime do criador (não do loader) e as fontes migraram para
  `assets::FontCache`, separando os dois lifetimes.
- `game_loop::FixedStepper` foi exposto como peça reutilizável do loop de 60 Hz.
- `src/main.rs` usa `bootstrap` + `Scene`; `examples/moving_sprite.rs` virou cena sobre `Engine`.
- `examples/mini_game.rs`: Pong jogável com menu, partida até 5 pontos, raquete por teclado/gamepad,
  colisão por `Rect`, `SceneManager::replace`, sons WAV gerados em runtime e carregados pelo
  `ResourceManager`, com fallback para `Sound::tone` e para execução silenciosa.

## Entregue na Fase 4

- `assets::loader` lê bytes com erros tipados (`InvalidPath`, `Io`), carrega texturas, sons,
  músicas e fontes, e diferencia falhas de decodificação de falhas de arquivo.
- `assets::ResourceManager` mantém caches de texturas, sons, músicas e fontes por `Rc`, com
  `CacheStats` (hits, misses, loads, requests), contadores por tipo e `clear()`.
- `assets::FontLoader` é dono do `Sdl2TtfContext`, resolve caminhos contra um diretório base e
  empresta fontes por tamanho; o cache usa a chave `(caminho, corpo)`.
- `audio::AudioSystem` abre/fecha o SDL_mixer, negocia a especificação com o backend e cresce os
  canais sob demanda sem reduzi-los.
- `audio::Sound` carrega arquivos, aceita amostras PCM de 16 bits (`from_samples`) e gera tons
  (`tone`), com volume normalizado em 0..1 e reprodução por canal.
- `audio::MusicTrack` encapsula música transmitida com `play`, `play_loop`, `pause`, `resume`,
  `stop`, `is_playing` e volume.
- `examples/audio_test.rs` sintetiza dois tons que reagem a Espaço/Enter e ao gamepad; sem
  dispositivo de áudio cai para o driver `dummy` e continua executando.
- Testes de integração cobrem cache de textura (`Rc::ptr_eq`, 1 load para 2 pedidos), cache de som
  com WAV gerado por `tempfile`, recurso ausente e fonte inexistente.

## Entregue na Fase 3

- `input::KeyboardState` distingue pressionado, mantido e liberado por quadro, ignora `KeyDown`
  repetido e oferece `movement()` normalizado para setas e WASD.
- `input::MouseState` acompanha posição, transições dos botões e rolagem acumulada por quadro.
- `input::GamepadState` abre o primeiro controle disponível, expõe botões com nomes neutros,
  sticks com zona morta configurável e gatilhos normalizados; ausência de controle não é erro.
- `input::Input` agrega os três dispositivos, limpa transições em `begin_frame` e aplica eventos
  com `update(&[Event])`.
- `platform::Sdl::poll_input` coleta eventos uma única vez, atualiza a entrada e informa o
  encerramento por Quit/Escape; `SdlEvent` e `poll_events` foram removidos.
- `Game::update(dt, &Input)` recebe o estado do quadro; o loop atualiza a entrada antes da
  simulação e mantém o passo fixo com acumulador.
- `examples/input_test.rs` demonstra movimento por setas/WASD/analógico, troca de cor com Espaço
  (ou botão sul do controle) e reposicionamento com o botão direito do mouse.
- `GameEngine::run` passou a receber o `Sdl` e não exige mais callback de encerramento manual.

## Fase 2 — Renderização 2D (concluída anteriormente)

- `RenderContext` implementa limpeza, retângulos, desenho de texturas com recorte, flip e rotação,
  e apresentação; falhas do SDL2 são propagadas por `RenderError`.
- `TextureLoader` mantém SDL_image ativo e permite carregar imagens PNG por caminho ou bytes, além
  de criar texturas RGBA a partir de pixels em memória.
- `Sprite` compartilha uma textura por `Rc` e desenha imagem inteira ou região de origem.
- `Camera` transforma pontos e retângulos entre coordenadas do mundo e da viewport, com testes de
  posição e zoom.
- `math::Color` tem construtores RGB/RGBA e `math::Rect` tem construtor e valor padrão.
- `hello_rect` desenha um retângulo que percorre uma órbita; `moving_sprite` demonstra textura
  procedural, sprite e movimento.

## Fase 1 — Platform e loop (concluída anteriormente)

- `platform::Sdl` mantém o contexto SDL2, subsistema de vídeo e fila de eventos; registra o driver
  ativo.
- `platform::Window` cria uma janela SDL2 com canvas e oferece dimensões, limpeza e apresentação.
- `Time::tick` mede delta, elapsed, FPS e frames, limitando delta acima de 250 ms.
- `game_loop::run` executa atualizações de passo fixo e renderização até Quit/Escape.
- O teste de integração cria uma janela com o driver SDL2 `dummy` e verifica o encerramento.

## Fase 0 — Bootstrap (concluída anteriormente)

- `Cargo.lock` é versionado; `.gitignore` não o exclui.
- O MSRV ficou em Rust 1.80 (dentro de `1.75+`), porque SDL2 0.38 não compila com Rust 1.75 ou
  1.77. A decisão e as verificações estão registradas em `docs/DECISIONS.md`.
- `actionlint` foi executado posteriormente (v1.7.7) em `ci.yml`, `release.yml` e
  `cross-compile.yml`: nenhum erro. O CI também valida compilação e testes.

## Validação em hardware (R35S / RK3326)

- Build cruzado contra o SDL2 do próprio aparelho: `--no-default-features --features sdl2-system`,
  com `PKG_CONFIG_PATH` apontando para um sysroot montado a partir do rootfs. O binário resultante
  exige no máximo `GLIBC_2.34` (aparelho tem 2.38) e apenas `libSDL2`/`libc`/`libgcc_s`.
- `SDL_VIDEODRIVER=kmsdrm` inicializou no console; o log confirmou `driver de vídeo SDL2 ativo:
  KMSDRM` e `gamepad conectado: r36s_Gamepad`.
- A sonda com `RenderContext::read_pixels` leu o quadro real: fundo `(20,60,140)` e retângulo
  `(240,90,40)` ocupando exatamente `x=300..419`, `y=180..299` (120x120 = 14.400 px).
- `SDL2_image` da imagem foi compilado **sem PNG**; a inicialização passou a ser sob demanda
  (`InitFlag::empty()`), o que manteve a engine funcional e transformou a limitação em erro do
  recurso específico.
- `SDL2_mixer` não existe no rootfs. Foi compilado do código-fonte (2.8.0, apenas WAV) e instalado
  em `/usr/lib` do aparelho; `audio_test` passou com `SDL_AUDIODRIVER=dummy`. **O SDL2 da imagem não
  tem backend ALSA** (somente `dsp`/`dummy`/`disk`), então áudio real exige ajuste no Buildroot.
- `/dev/fb0` (`rockchipdrmfb`) não espelha o scanout do KMSDRM; a captura deve usar `read_pixels`.
- Fase 4 no aparelho: `audio_test` compilado com `sdl2-system` abriu KMSDRM, detectou o gamepad e,
  sem backend ALSA, caiu para `SDL_AUDIODRIVER=dummy` mantendo-se ativo. Uma sonda dedicada
  confirmou `AudioSpec { frequency: 44100, channels: 1 }`, 4 canais alocados, `playing_channels()`
  passando a 1 após `Sound::tone(...).play()` e erro correto para arquivo de áudio ausente.
- Fase 5 no aparelho (R35S, KMSDRM): `giochi-demo` e `mini_game` compilados com `sdl2-system`
  abriram KMSDRM e permaneceram ativos; o log mostrou `driver de vídeo SDL2 ativo: KMSDRM`,
  `gamepad conectado: r36s_Gamepad` e o fallback esperado de áudio
  (`áudio indisponível, seguindo sem som`) porque a imagem não tem backend ALSA.
- A captura via `Engine::read_pixels` (nova API de screenshot) em um quadro da `Engine` com cena
  comprovou o desenho: fundo `(12,16,24)`, raquete esquerda `(90,220,160)` em x=32..45, raquete
  direita `(240,160,90)` em x=594..607, bola branca `(245,245,245)` em x≈314..328, placar e rede
  presentes.
- Após os testes, `/etc/inittab` foi restaurado e o aparelho reiniciado; o launcher voltou a rodar.

## Cadeia `cross` validada localmente

- `qemu-aarch64` registrado via `docker run --privileged --rm tonistiigi/binfmt --install arm64`
  permitiu rodar a cadeia real: `Cross.toml` construiu a imagem com as dependências ARM64 e
  `cross build --locked --release --target aarch64-unknown-linux-gnu` concluiu com sucesso.
- Achado importante: o SDL2 `bundled` gerado pelo `cross` é uma shared library ligada a
  `libsndio`, `libwayland-*`, `libxkbcommon`, `libdecor`, `libjack`, `libpipewire`, `libpulse` e
  `libsamplerate`. Testado no R35S, o binário falhou com
  `error while loading shared libraries: libsndio.so.7: cannot open shared object file`.
- Correção: o release não empacota essa biblioteca; o binário usa o SDL2 do sistema
  (`>= 2.26` com KMSDRM). O mesmo binário do `cross`, executado no R35S com o SDL2 2.28.5 da
  imagem, abriu KMSDRM e detectou o `r36s_Gamepad`.
- O pacote final (`giochi-engine-v0.1.0-test-aarch64-unknown-linux-gnu.tar.gz`) foi enviado ao
  aparelho: `giochi-demo`, `giochi-hello_rect` e `giochi-mini_game` ficaram ativos em KMSDRM.
- O passo de verificação do workflow também foi exercitado: detectou uma `libSDL2` x86-64 que
  havia entrado no pacote e falhou como deveria.

## Manutenção pós-0.1.0 (aplicada)

- `v0.1.0` tagueada em `1100e17`; o workflow de Release executou a partir da tag.
- Dependências sem uso removidas (`serde`, `serde_yaml`, `approx`): o grafo direto caiu para 9
  entradas e o build não compila mais `serde_derive`/`jiff`.
- `.github/dependabot.yml`: `ignore` para `dtolnay/rust-toolchain` (o pin é o gate de MSRV) e para
  `glam` minor/major (0.34 exige Cargo > 1.80); agrupamento de atualizações patch/minor para
  reduzir ruído.
- PRs do Dependabot #1 (rust-toolchain 1.120.0) e #5 (glam 0.34.2) fechados por quebrarem o gate.
- Relatório técnico movido da raiz para `docs/relatorio-tecnico-arm64.md`, com nome normalizado e
  nota indicando que é documento histórico.

## Bloqueios / pendências

- A imagem do GIOCHI KONSOL OS não traz backend ALSA no SDL2, então o áudio real continua
  indisponível no aparelho (a engine degrada para silêncio).
- Cobertura unitária ampliada: `math` (interseção, limites, cor) e `assets` (`CacheStats`,
  `AssetManager`, `clear`) ganharam testes próprios. `platform` continua coberto apenas pela
  integração, porque o SDL2 admite um único contexto por processo e os testes de janela já
  serializam esse uso. Não há medição de cobertura no CI.
- PRs #2/#3/#4 do Dependabot (checkout, upload-artifact, setup-qemu) seguem abertos; exigem
  Node 24 no runner, por isso o CI emite aviso de deprecação do Node 20.
- Habilitar ALSA e PNG no Buildroot do GIOCHI KONSOL OS para que áudio real e assets PNG funcionem
  sem bibliotecas extras.
- O floating point de gamepad/sticks usa os eixos crus do SDL2; mapeamentos específicos de
  hardware (ex.: inversão de eixo) ainda podem ser adicionados em uma fase de polimento.

## Roadmap pós-0.1.0

Fases 0 a 6 concluídas. Próximos incrementos sugeridos, fora do escopo original:

- Renderização de texto com `FontCache` + `RenderContext` (helper `draw_text`).
- Animação de sprites por quadros (`AnimationClip`, `Animator`).
- Física 2D simples (AABB com resposta, camadas) sobre `math::Rect`.
- Áudio posicional/pan e controle por canal no `AudioSystem`.
- Empacotamento (`.tar.gz` por distro, `AppImage`/Buildroot package) e publicação no crates.io.
