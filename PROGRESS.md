# Progresso da implementação

## Última fase concluída: Fase 4 — Assets e áudio

Data: 2026-11-06

## Fases

| Fase | Estado | Resumo |
|---|---|---|
| Fase 0 — Bootstrap | Concluída | Pacote Cargo, binário de smoke test, módulos públicos, toolchain, Cross, documentação, workflows e testes iniciais. |
| Fase 1 — Platform e loop | Concluída | Inicialização SDL2 e vídeo, wrapper de janela, tempo delta/elapsed/fps e loop fixo com saída por Quit/Escape. |
| Fase 2 — Renderização 2D | Concluída | Primitivas, texturas SDL_image/RGBA, sprites, câmera e exemplos funcionais com contexto próprio de renderização. |
| Fase 3 — Input | Concluída | Teclado, mouse e gamepad com estados por quadro, integração no loop e exemplo `input_test`. |
| Fase 4 — Assets e áudio | Concluída | Cache de texturas/sons/músicas/fontes, `AudioSystem`, efeitos, música e exemplo `audio_test`. |
| Fase 5 — Cenas e integração | Pendente | Implementar gerenciamento de cenas e fachada unificada da engine. |
| Fase 6 — Polimento e release | Pendente | Finalizar CI, documentação de produção, validação ARM64 e release. |

## Gates validados na Fase 4

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo build --locked --target x86_64-unknown-linux-gnu --all-targets`
- [x] `cargo test --locked --target x86_64-unknown-linux-gnu` (26 testes: 16 unitários e 10 de
  integração; cache de textura e de som com duas requisições por recurso, reprodução de tom no
  driver `dummy`, além dos testes de entrada e renderização por pixel)
- [x] `cargo check --locked --target aarch64-unknown-linux-gnu --all-targets` — passou com o
  `aarch64-linux-gnu-gcc` instalado no host. A cadeia `cross` com QEMU/binfmt continua validada
  apenas pelo workflow.
- [x] `cargo doc --locked --no-deps --document-private-items`
- [x] Execução em hardware ARM64 real (R35S/RK3326, GIOCHI KONSOL OS, Buildroot 2024.02,
  kernel 6.12): driver `KMSDRM` ativo, `/dev/dri/card0` aberto, gamepad `r36s_Gamepad` detectado e
  os exemplos `hello_rect`, `moving_sprite` e `input_test` ativos em console sem X11/Wayland.

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
- Após os testes, `/etc/inittab` foi restaurado e o aparelho reiniciado; o launcher voltou a rodar.

## Bloqueios / pendências

- Validar a cadeia `cross` com QEMU/binfmt no workflow (a validação local usou o sysroot do
  aparelho, que não depende de emulação).
- Habilitar ALSA e PNG no Buildroot do GIOCHI KONSOL OS para que áudio real e assets PNG funcionem
  sem bibliotecas extras.
- O floating point de gamepad/sticks usa os eixos crus do SDL2; mapeamentos específicos de
  hardware (ex.: inversão de eixo) ainda podem ser adicionados em uma fase de polimento.

## Próxima fase: Fase 5 — Cenas e integração

Implementar `Scene`/`SceneManager` com pilha e transições, uma fachada `Engine` que possua `Sdl`,
`Window`, `Input`, `ResourceManager` e `AudioSystem`, e um `examples/mini_game.rs` jogável usando
todos os módulos.
