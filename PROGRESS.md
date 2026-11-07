# Progresso da implementação

## Última fase concluída: Fase 3 — Input

Data: 2026-11-06

## Fases

| Fase | Estado | Resumo |
|---|---|---|
| Fase 0 — Bootstrap | Concluída | Pacote Cargo, binário de smoke test, módulos públicos, toolchain, Cross, documentação, workflows e testes iniciais. |
| Fase 1 — Platform e loop | Concluída | Inicialização SDL2 e vídeo, wrapper de janela, tempo delta/elapsed/fps e loop fixo com saída por Quit/Escape. |
| Fase 2 — Renderização 2D | Concluída | Primitivas, texturas SDL_image/RGBA, sprites, câmera e exemplos funcionais com contexto próprio de renderização. |
| Fase 3 — Input | Concluída | Teclado, mouse e gamepad com estados por quadro, integração no loop e exemplo `input_test`. |
| Fase 4 — Assets e áudio | Pendente | Integrar cache/carregamento de recursos, mixer, efeitos sonoros e música. |
| Fase 5 — Cenas e integração | Pendente | Implementar gerenciamento de cenas e fachada unificada da engine. |
| Fase 6 — Polimento e release | Pendente | Finalizar CI, documentação de produção, validação ARM64 e release. |

## Gates validados na Fase 3

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo build --locked --target x86_64-unknown-linux-gnu --all-targets`
- [x] `cargo test --locked --target x86_64-unknown-linux-gnu` (17 testes: 14 unitários e 3 integração;
  entrada coberta com eventos SDL2 sintéticos para teclado, mouse e gamepad)
- [x] `cargo check --locked --target aarch64-unknown-linux-gnu --all-targets` — passou com o
  `aarch64-linux-gnu-gcc` instalado no host. A cadeia `cross` com QEMU/binfmt continua validada
  apenas pelo workflow.
- [x] `cargo doc --locked --no-deps --document-private-items`
- [ ] Executar `input_test` em desktop ou hardware KMSDRM — ambiente sem display; a lógica de
  entrada é coberta por testes unitários com eventos SDL2 sintéticos.

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
- `actionlint` não estava instalado no ambiente; os workflows foram revisados manualmente e o CI
  valida compilação e testes.

## Bloqueios / pendências

- Validar a cadeia `cross` com QEMU/binfmt no workflow e executar os exemplos em um ambiente com
  display ou hardware KMSDRM.
- `actionlint` não está instalado neste ambiente.
- O floating point de gamepad/sticks usa os eixos crus do SDL2; mapeamentos específicos de
  hardware (ex.: inversão de eixo) ainda podem ser adicionados em uma fase de polimento.

## Próxima fase: Fase 4 — Assets e áudio

Integrar o cache de recursos a um carregador de texturas com `Rc`, adicionar fontes/SDL_ttf e
implementar o `AudioSystem` sobre SDL2_mixer, com efeitos sonoros, música e exemplo de áudio.
