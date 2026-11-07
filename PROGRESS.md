# Progresso da implementação

## Última fase concluída: Fase 2 — Renderização 2D

Data: 2026-11-06

## Fases

| Fase | Estado | Resumo |
|---|---|---|
| Fase 0 — Bootstrap | Concluída | Pacote Cargo, binário de smoke test, módulos públicos, toolchain, Cross, documentação, workflows e testes iniciais. |
| Fase 1 — Platform e loop | Concluída | Inicialização SDL2 e vídeo, wrapper de janela, tempo delta/elapsed/fps e loop fixo com saída por Quit/Escape. |
| Fase 2 — Renderização 2D | Concluída | Primitivas, texturas SDL_image/RGBA, sprites, câmera e exemplos funcionais com contexto próprio de renderização. |
| Fase 3 — Input | Pendente | Integrar teclado, mouse e gamepad aos eventos SDL2, com estados por frame. |
| Fase 4 — Assets e áudio | Pendente | Integrar cache/carregamento de recursos, mixer, efeitos sonoros e música. |
| Fase 5 — Cenas e integração | Pendente | Implementar gerenciamento de cenas e fachada unificada da engine. |
| Fase 6 — Polimento e release | Pendente | Finalizar CI, documentação de produção, validação ARM64 e release. |

## Gates validados na Fase 2

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo build --locked --target x86_64-unknown-linux-gnu --all-targets`
- [x] `cargo test --locked --target x86_64-unknown-linux-gnu` (7 testes: 4 unitários e 3 integração;
  os testes headless criam/desenham retângulo e textura usando o driver SDL2 `dummy`)
- [ ] `cargo check --target aarch64-unknown-linux-gnu` — target presente, mas o check direto não
  encontra o compilador C cruzado; `cross check` bloqueia na preparação das dependências ARM por
  falta de QEMU/binfmt no host. Os workflows instalam QEMU antes de executar Cross.
- [x] `cargo doc --locked --no-deps --document-private-items`
- [ ] Executar os exemplos em desktop ou hardware KMSDRM — ambiente atual sem display; a
  renderização foi exercitada em modo headless com SDL2 `dummy`.

## Entregue na Fase 2

- `RenderContext` implementa limpeza, retângulos, desenho de texturas com recorte, flip e rotação,
  e apresentação; falhas do SDL2 são propagadas por `RenderError`.
- `TextureLoader` mantém SDL_image ativo e permite carregar imagens PNG por caminho ou bytes, além
  de criar texturas RGBA a partir de pixels em memória.
- `Sprite` compartilha uma textura por `Rc` e desenha imagem inteira ou região de origem.
- `Camera` transforma pontos e retângulos entre coordenadas do mundo e da viewport, com testes de
  posição e zoom.
- `math::Color` tem construtores RGB/RGBA, `math::Rect` tem construtor e valor padrão; ambos são
  independentes de SDL2.
- `hello_rect` desenha um retângulo que percorre uma órbita; `moving_sprite` demonstra textura
  procedural, sprite e controle por setas.

## Fase 1 — Platform e loop (concluída anteriormente)

- `platform::Sdl` mantém o contexto SDL2, subsistema de vídeo e fila de eventos; registra o driver
  ativo e converte Quit/Escape em eventos da plataforma.
- `platform::Window` cria uma janela SDL2 com canvas e oferece dimensões, limpeza e apresentação.
- `Time::tick` mede delta, elapsed, FPS e frames, limitando delta acima de 250 ms.
- `game_loop::run` executa atualizações de passo fixo e renderização até Quit/Escape; o demo abre
  janela de 1280x720.
- O teste de integração cria uma janela com o driver SDL2 `dummy`, enfileira Escape e verifica o
  encerramento do loop.

## Fase 0 — Bootstrap (concluída anteriormente)

- O repositório já continha implementação além dos stubs previstos no bootstrap; essa base foi
  preservada.
- `Cargo.lock` é versionado; `.gitignore` não o exclui.
- O MSRV ficou em Rust 1.80 (dentro de `1.75+`), porque SDL2 0.38 não compila com Rust 1.75 ou
  1.77. A decisão e as verificações estão descritas em `docs/DECISIONS.md`.
- `actionlint` não estava instalado no ambiente; os arquivos de workflow foram revisados e o CI
  valida os comandos de compilação e testes.

## Bloqueios / pendências

- Validar cross-compilation ARM64 no workflow com QEMU/binfmt e executar os exemplos em um ambiente
  com display ou hardware KMSDRM.
- `actionlint` não está instalado neste ambiente.
- A camada `platform` ainda oferece acesso aos eventos SDL brutos; a próxima fase deve fornecer a
  API portátil de input e migrar os exemplos para ela.

## Próxima fase: Fase 3 — Input

Implementar estados por frame de teclado e mouse, suporte inicial a gamepad e integração de eventos
ao loop; acrescentar testes para pressed/held/released e atualizar o exemplo de controle.
