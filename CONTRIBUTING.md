# Contribuindo

Obrigado por contribuir com o GIOCHI-ENGINE e o ecossistema GIOCHI-SYSTEM.

## Issues

Abra uma issue descrevendo o problema ou a proposta, passos para reproduzir, sistema operacional,
target Rust, versão do SDL2 e, para falhas de hardware, modelo da placa e firmware. Não inclua
informações privadas ou credenciais.

## Commits

Use [Conventional Commits](https://www.conventionalcommits.org/), por exemplo:

- `feat(render): adicionar desenho de sprites`
- `fix(platform): preservar driver SDL configurado`
- `docs(kmsdrm): documentar acesso à TTY`

## Checklist de pull request

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo doc --no-deps` sem avisos novos
- [ ] `actionlint` sem erros, se workflows do GitHub Actions foram alterados
- [ ] Documentação pública atualizada em português brasileiro.
- [ ] Mudanças de plataforma verificadas em x86_64 ou por cross-compilation.
- [ ] Comportamento KMSDRM testado em hardware ARM64 real quando a alteração envolver vídeo,
  entrada, áudio ou dependências nativas.

Explique limitações de hardware e forneça os comandos/resultados dos testes no pull request.

## Recursos e assets

Não comite binários grandes. `assets/**/*.png` está no `.gitignore`; mantenha apenas o
`assets/.gitkeep`. Prefira texturas procedurais nos exemplos, como faz `moving_sprite`.
