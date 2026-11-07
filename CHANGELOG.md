# Registro de alterações

## [Não lançado]

- Camada de renderização 2D: primitivas, texturas SDL_image, sprites e câmera mundo/tela.
- Camada de entrada unificada: teclado, mouse e gamepad com transições por quadro.
- Captura de pixels (`RenderContext::read_pixels`) para testes headless e evidência em hardware.
- Feature `sdl2-system` para compilar contra o SDL2 da imagem embarcada.
- Inicialização do SDL_image tolerante a builds sem PNG/JPEG.

## [0.1.0] - 2026-11-06

- Estrutura inicial da biblioteca e do demo SDL2.
- Loop de jogo de timestep fixo, adaptador de janela e tipos iniciais para os módulos 2D.
- Documentação de build nativo, KMSDRM e cross-compilation ARM64.
