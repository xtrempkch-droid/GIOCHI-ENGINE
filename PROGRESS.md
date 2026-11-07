# Progresso da implementação

## Última fase concluída: Fase 1 — Plataforma SDL2 e loop básico

Data: 2026-11-06

## Fases

| Fase | Estado | Resumo |
|---|---|---|
| Fase 0 — Bootstrap | Concluída | Pacote Cargo, binário de smoke test, módulos públicos, toolchain, Cross, documentação, workflows e testes iniciais. |
| Fase 1 — Plataforma e loop | Concluída | Inicialização SDL2 e vídeo, wrapper de janela, tempo delta/elapsed/fps e loop fixo com saída por Quit/Escape. |
| Fase 2 — Plataforma SDL2/KMSDRM | Pendente | Evoluir inicialização SDL2, janela e validação em TTY/ARM64. |
| Fase 3 — Input | Pendente | Integrar teclado, mouse e gamepad aos eventos SDL2. |
| Fase 4 — Renderização e matemática | Pendente | Completar renderer, texturas, sprites, câmera e primitivas 2D. |
| Fase 5 — Assets, áudio e cenas | Pendente | Implementar carregamento/cache, mixer e transições de estado/cena. |
| Fase 6 — Integração GIOCHI-SYSTEM | Pendente | Validar API consumidora, empacotamento e releases ARM64. |

## Gates validados na Fase 1

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo build --locked --target x86_64-unknown-linux-gnu --all-targets`
- [x] `cargo test --locked --target x86_64-unknown-linux-gnu` (5 testes: 3 integração e 2 de tempo,
  incluindo encerramento do loop com Escape no driver headless `dummy`)
- [ ] `cargo check --target aarch64-unknown-linux-gnu` — target presente, mas o check direto não
  encontra o compilador C cruzado; `cross check` bloqueia na preparação das dependências ARM por
  falta de QEMU/binfmt no host. Os workflows instalam QEMU antes de executar Cross.
- [x] `cargo doc --locked --no-deps --document-private-items`
- [x] `game_loop_exits_when_escape_is_queued` no driver headless `dummy`; execução visual em
  hardware ou desktop SDL2 permanece pendente porque o ambiente atual não tem display.

## O que foi entregue na Fase 1

- `platform::Sdl` mantém o contexto SDL2, subsistema de vídeo e fila de eventos; registra o driver
  ativo e converte Quit/Escape em eventos da plataforma.
- `platform::Window` cria uma janela SDL2 com canvas e oferece dimensões, limpeza e apresentação.
- `Time::tick` mede delta, elapsed, FPS e frames, limitando delta acima de 250 ms; testes unitários
  cobrem o delta e o clamp.
- `game_loop::run` executa atualizações de passo fixo e renderização até Quit/Escape; o demo abre
  janela preta de 1280x720.
- O teste de integração cria uma janela com o driver SDL2 `dummy`, enfileira Escape e verifica que o
  loop encerra sem executar atualização ou renderização.

## Fase 0 — Bootstrap (concluída anteriormente)

- O repositório já continha implementação além dos stubs previstos no bootstrap. Essa base foi
  preservada; os exemplos independentes mantêm as demonstrações de movimento e áudio.
- `Cargo.lock` é versionado; `.gitignore` não o exclui.
- O MSRV ficou em Rust 1.80 (dentro de `1.75+`), porque SDL2 0.38 não compila com Rust 1.75 ou
  1.77. A decisão e as verificações estão descritas em `docs/DECISIONS.md`.
- Cross-check ARM64 local permanece sem confirmação: a imagem Cross exige emulação ARM64 para
  executar scripts de instalação multiarch. Os workflows habilitam QEMU/binfmt para essa etapa.
- `actionlint` não estava instalado no ambiente; os arquivos de workflow foram revisados e o CI
  valida os comandos de compilação e testes.

## Próxima fase: Fase 2 — Renderização 2D

Completar primitivas de cor/retângulo, texturas SDL_image, desenho de sprites e transformações de
câmera. Acrescentar testes da conversão mundo/tela e exemplos SDL2 funcionais; validar a execução
em desktop x86_64.

## Bloqueios / pendências

- Validar cross-compilation ARM64 no workflow com QEMU/binfmt e executar o demo em um ambiente com
  display ou hardware KMSDRM.
- `actionlint` não está instalado neste ambiente.

## Notas de arquitetura

- Os eventos SDL brutos continuam acessíveis pela camada `platform` para os exemplos existentes;
  o loop da engine usa somente `SdlEvent`. A próxima fase de input deverá substituir esse acesso
  direto nos exemplos.
