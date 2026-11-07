# Decisões de arquitetura

## Dependências e targets

- A engine é um único pacote Cargo com biblioteca `giochi_engine` e binário `giochi-demo`, para
  permitir consumo como crate e validação executável no mesmo repositório.
- SDL2 principal usa o recurso `bundled`; extensões SDL2_image, SDL2_mixer e SDL2_ttf usam os
  pacotes do sistema, pois essa configuração aproveita o backend SDL2 distribuído no Linux.
- `bundled` reduz a dependência de uma instalação prévia do SDL2 principal e mantém uma versão
  previsível entre desenvolvimento e compilação cruzada; as extensões continuam usando pacotes
  nativos da plataforma.
- `glam` foi escolhido para vetores e transformações 2D por ser leve, orientado a jogos e oferecer
  tipos vetorizados compatíveis com os casos de uso da engine; `nalgebra` seria mais amplo que o
  necessário para a matemática 2D prevista.
- x86_64 Linux é o alvo de desenvolvimento local; ARM64 Linux é o alvo embarcado e usa `cross`.
- `Cargo.lock` deve ser versionado por se tratar de aplicação/binário, ainda que crate reutilizável.
- O MSRV efetivo é Rust 1.80: SDL2 0.38 usa `std::mem::offset_of!` e `size_of` no prelude,
  indisponíveis em Rust 1.75. Testes com Rust 1.75 e 1.77 falham dentro da dependência SDL2;
  o primeiro toolchain compatível validado é 1.80.

## Organização e tempo

- Os módulos são separados por responsabilidade; SDL2 fica em `platform`, tipos de renderização
  e matemática não dependem diretamente do SDL2.
- O loop usa timestep fixo de 1/60 s, acumulador e limite de 250 ms de tempo por quadro. A limitação
  evita que uma pausa longa provoque uma quantidade ilimitada de atualizações.
- A camada `platform` encapsula o `EventPump`, entrega os eventos à camada `input` e decide o
  encerramento por Quit/Escape; `core::game_loop` trata somente a API da plataforma, a entrada e
  o contexto de renderização.
- KMSDRM é selecionado somente quando `SDL_VIDEODRIVER` não foi definido, preservando a escolha
  explícita de drivers desktop ou de configuração do usuário.
- `RenderContext` é implementado sobre SDL2 em `render`, enquanto `Window` permanece na camada
  `platform`; jogos usam apenas tipos de cor/retângulo próprios e erros tipados.
- `Window` possui o `TextureCreator` SDL2; `render_context_with_loader` empresta o canvas e a
  fábrica ao mesmo tempo, e o loader mantém SDL_image ativo. Texturas têm lifetime vinculado ao
  loader, impedindo que sobrevivam ao canvas que as criou.
- `Game::render` retorna `RenderError`, propagado pelo loop ao chamador, para não ocultar falhas de
  desenho.
- A entrada é orientada a eventos e consolidada por quadro: `begin_frame` limpa as transições e
  `update(&[Event])` aplica os eventos pendentes. `is_down` representa o estado mantido,
  `is_pressed_this_frame` e `is_released_this_frame` representam as transições do quadro.
- Eventos `KeyDown` repetidos do SDL2 são ignorados para não reativar ações disparadas por borda.
- `Input::update` recebe `sdl2::event::Event` porque a camada `input` é o adaptador de backend; os
  jogos consultam apenas enums próprios (`MouseButton`, `GamepadButton`) e `math::Vec2`.
- O gamepad é opcional: falhas ao abrir o subsistema ou o dispositivo geram log e a engine segue
  sem controle. O primeiro controle conectado é aberto automaticamente e pode ser substituído
  quando um novo dispositivo é adicionado.
- A zona morta padrão dos analógicos é 0.2, configurável com `GamepadState::set_deadzone`. O
  controle é declarado antes do subsistema no struct para ser liberado primeiro.

## Estado da implementação

- Implementados: inicialização SDL2, janela/canvas, loop fixo, renderização de primitivas e
  texturas, sprites, câmera mundo/tela, entrada de teclado/mouse/gamepad e estruturas básicas de
  matemática/cache/estado.
- Base parcial: cenas, cache de texturas e áudio. A textura já usa SDL_image e o gamepad usa
  SDL2_controller; a integração do cache de recursos e do mixer continua nas próximas fases.
- Erros de biblioteca usam `thiserror`; aplicações podem agregar contexto com `anyhow`.
- `thiserror` fornece erros tipados e sem dependência de contexto de aplicação na biblioteca;
  `anyhow` facilita a composição e apresentação de erros no binário e nos exemplos.
- O binário principal permanece uma demonstração SDL2 mínima de janela preta e saída por Escape;
  os exemplos independentes demonstram retângulo animado, sprite controlado pela entrada e o
  agregado `Input` com teclado, mouse e gamepad.
