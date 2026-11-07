# Progresso da implementação

## Estado da sessão

- Última fase concluída: **Fase 0 — Bootstrap do repositório**.
- Próxima fase: **Fase 1 — Ciclo de vida da engine e loop de jogo**.
- Data de atualização: 2026-11-06.

## Fases

| Fase | Estado | Resumo |
|---|---|---|
| Fase 0 — Bootstrap | Concluída | Pacote Cargo, binário de smoke test, módulos públicos, toolchain, Cross, documentação, workflows e testes iniciais. |
| Fase 1 — Core e loop | Pendente | Consolidar a fachada `Engine`, ciclo de vida e testes do timestep fixo. |
| Fase 2 — Plataforma SDL2/KMSDRM | Pendente | Evoluir inicialização SDL2, janela e validação em TTY/ARM64. |
| Fase 3 — Input | Pendente | Integrar teclado, mouse e gamepad aos eventos SDL2. |
| Fase 4 — Renderização e matemática | Pendente | Completar renderer, texturas, sprites, câmera e primitivas 2D. |
| Fase 5 — Assets, áudio e cenas | Pendente | Implementar carregamento/cache, mixer e transições de estado/cena. |
| Fase 6 — Integração GIOCHI-SYSTEM | Pendente | Validar API consumidora, empacotamento e releases ARM64. |

## Entrega da Fase 0

- O repositório já continha uma implementação mais completa que os stubs previstos no bootstrap.
  Essa funcionalidade foi preservada; `giochi-demo` agora é o smoke test solicitado e os exemplos
  independentes mantêm as demonstrações gráficas e de áudio.
- `Cargo.lock` é versionado; `.gitignore` não o exclui.
- O MSRV ficou em Rust 1.80 (dentro de `1.75+`), porque SDL2 0.38 não compila com Rust 1.75 ou
  1.77. A decisão e as verificações estão descritas em `docs/DECISIONS.md`.
- Cross-check ARM64 local permanece sem confirmação: a imagem Cross exige emulação ARM64 para
  executar scripts de instalação multiarch. Os workflows habilitam QEMU/binfmt para essa etapa.
- `actionlint` não estava instalado no ambiente; os arquivos de workflow foram revisados e o CI
  valida os comandos de compilação e testes.

## Gates da Fase 0

- [x] `cargo fmt --all -- --check`
- [x] `cargo clippy --locked --all-targets -- -D warnings`
- [x] `cargo build --locked --target x86_64-unknown-linux-gnu --all-targets`
- [x] `cargo test --locked --target x86_64-unknown-linux-gnu` (2 testes headless)
- [x] `cargo doc --locked --no-deps --document-private-items`
- [x] `cargo run --locked --bin giochi-demo` imprime `giochi-engine bootstrap OK`
- [ ] `cross check --target aarch64-unknown-linux-gnu` — bloqueado no host por ausência de
  emulação ARM64; verificar pelo workflow com QEMU ou em ambiente ARM64.
