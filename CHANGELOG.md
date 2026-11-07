# Registro de alterações

## [Não lançado]

- Fachada `core::Engine`: reúne SDL2, janela, entrada, cache de recursos, áudio e pilha de cenas;
  `Engine::run` executa o loop fixo e `Engine::step` permite controle quadro a quadro.
- Cenas: `Scene` (on_enter/on_exit/update/render/handle_event), `SceneContext` e `SceneManager`
  com `push`, `pop`, `replace`, `current`, `current_mut` e aplicação de transições entre quadros.
- `mini_game`: Pong jogável usado como teste de fumaça de todos os módulos, com sons gerados em
  tempo de execução e carregados pelo `ResourceManager`.
- `Window::split`/`RenderTarget` e `RenderContext::ensure_cleared`; texturas agora têm o lifetime
  do criador, não do loader; `FontCache` separado do `ResourceManager`.
- `EngineConfig`, `EngineError` e `bootstrap` para inicialização em uma chamada.
- Assets: `ResourceManager` com cache de texturas, sons, músicas e fontes, estatísticas de
  acerto/falta e erros tipados de caminho, leitura e decodificação.
- Áudio: `AudioSystem`, `Sound` (arquivo ou amostras PCM) e `MusicTrack` sobre SDL2_mixer, com
  exemplo `audio_test` que sintetiza tons e reage à entrada.
- Camada de renderização 2D: primitivas, texturas SDL_image, sprites e câmera mundo/tela.
- Camada de entrada unificada: teclado, mouse e gamepad com transições por quadro.
- Captura de pixels (`RenderContext::read_pixels`) para testes headless e evidência em hardware.
- Feature `sdl2-system` para compilar contra o SDL2 da imagem embarcada.
- Inicialização do SDL_image tolerante a builds sem PNG/JPEG.

## [0.1.0] - 2026-11-06

- Estrutura inicial da biblioteca e do demo SDL2.
- Loop de jogo de timestep fixo, adaptador de janela e tipos iniciais para os módulos 2D.
- Documentação de build nativo, KMSDRM e cross-compilation ARM64.
