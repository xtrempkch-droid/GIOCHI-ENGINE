# Decisões de arquitetura

## Dependências e targets

- A engine é um único pacote Cargo com biblioteca `giochi_engine` e binário `giochi-demo`, para
  permitir consumo como crate e validação executável no mesmo repositório.
- SDL2 principal usa o recurso `bundled`; extensões SDL2_image, SDL2_mixer e SDL2_ttf usam os
  pacotes do sistema, pois essa configuração aproveita o backend SDL2 distribuído no Linux.
- `bundled` reduz a dependência de uma instalação prévia do SDL2 principal e mantém uma versão
  previsível entre desenvolvimento e compilação cruzada; as extensões continuam usando pacotes
  nativos da plataforma.
- O SDL2 é uma dependência opcional por feature: `sdl2-bundled` (padrão) compila o SDL2 e
  `sdl2-system` usa `use-pkgconfig`. A separação existe porque imagens Buildroot embarcadas já
  trazem SDL2 compilado com KMSDRM, e o SDL2 `bundled` cruzado tende a desabilitar KMSDRM quando
  faltam headers de `libdrm`/`libgbm` no sysroot. Validado no R35S: o build `sdl2-system` gerou um
  binário que exigiu apenas `GLIBC_2.34` (aparelho tem 2.38).
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
- O SDL_image é inicializado com `InitFlag::empty()`: a versão 2.x carrega decodificadores sob
  demanda. Exigir PNG no `init` quebrava em imagens Buildroot compiladas sem libpng, mesmo quando
  os assets eram procedurais; a ausência de um formato agora vira erro de carregamento do recurso.
- `Window` possui o `TextureCreator` SDL2; `render_context_with_loader` empresta o canvas e a
  fábrica ao mesmo tempo, e o loader mantém SDL_image ativo. Texturas têm lifetime vinculado ao
  loader, impedindo que sobrevivam ao canvas que as criou.
- `Game::render` retorna `RenderError`, propagado pelo loop ao chamador, para não ocultar falhas de
  desenho.
- `RenderContext::read_pixels` expõe RGBA8888 do quadro atual para capturas e para testes de pixel
  headless, permitindo validar renderização no CI e em hardware sem framebuffer legado.
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

## Dependências

- `serde`, `serde_yaml` e `approx` foram **removidos** em 0.1.0: estavam declarados na
  especificação inicial, mas nenhum módulo os usava. Mantê-los compilava `serde_derive` e `jiff`
  em todo build, aumentava a superfície de dependências e não agregava comportamento. Quando
  houver configuração de jogo em YAML (roadmap), a dependência volta junto do uso real.
- O pin `dtolnay/rust-toolchain@1.80.0` no job de MSRV está protegido por `ignore` no
  Dependabot: ele É o gate de versão mínima, e o bot propõe versões inexistentes (`1.120.0`).
  `glam` também está ignorado em minor/major até que o MSRV seja elevado de forma deliberada,
  porque `glam 0.34` exige um Cargo mais novo que o 1.80.

## Release e empacotamento

- O release usa `cross` para produzir binário e exemplos AArch64, mas **não distribui o
  `libSDL2-2.0.so.0` do `bundled`**: medido no artefato, ele depende de `libsndio`, `libwayland-*`,
  `libxkbcommon`, `libdecor`, `libjack`, `libpipewire`, `libpulse` e `libsamplerate`, ausentes em
  imagens Buildroot. Um teste real no R35S confirmou a falha de carregamento
  (`libsndio.so.7: cannot open shared object file`) e, sem essa biblioteca no pacote, o mesmo
  binário roda com o SDL2 do sistema.
- Tentativa descartada: desligar os backends de desktop via `SDL2_TOOLCHAIN` + arquivo CMake
  próprio. O `sdl2-sys` substitui `CMAKE_TOOLCHAIN_FILE`, o que perde o toolchain do `cross`
  (CMake passa a usar o compilador nativo e gera uma `libSDL2` x86-64 dentro do build ARM64).
  Encadear o toolchain por `include($ENV{CMAKE_TOOLCHAIN_FILE})` não restaurou o compilador.
  Como a opção `sdl2-system` já produz o artefato correto e está validada em hardware, o caminho
  foi removido em vez de mantido como mecanismo frágil.
- O passo de verificação do release falha se algum arquivo do pacote não for `ARM aarch64` ou se
  houver qualquer binário `x86-64` — foi essa checagem que detectou a poluição acima.
- `--lib --bins --examples` (e não `--all-targets`) no release: testes não vão para o pacote e a
  `rlib` é reunida explicitamente, com fallback para `deps/`.
- O workflow aceita `workflow_dispatch` para gerar o pacote como artefato de pré-visualização sem
  publicar Release, permitindo validar empacotamento antes de criar a tag.

## Fachada, cenas e lifetimes

- `Engine::new(sdl, &mut window, config)` recebe a janela emprestada em vez de criá-la. A
  alternativa (`new(config)`) exigiria guardar a janela e o cache de texturas na mesma struct, o
  que é auto-referência: as texturas emprestam o `TextureCreator` da janela. Resolver isso exigiria
  `unsafe`, `Box::leak` ou a feature `unsafe_textures` (que torna a destruição de texturas depois
  do renderer um risco em tempo de execução). O empréstimo mantém a garantia verificada pelo
  compilador e foi validado em hardware; `Engine::bootstrap(config, scene)` cobre o fluxo
  "inicializa tudo e roda" sem expor lifetimes ao chamador.
- `Window::split` devolve `RenderTarget` (canvas mutável) e `&TextureCreator` (imutável) a partir de
  campos distintos. É isso que permite `Engine` guardar alvo e cache simultaneamente.
- O lifetime de `Texture` é o do `TextureCreator`, não o do `TextureLoader`. A textura pertence ao
  renderer; o SDL_image só decodifica superfícies. Antes, guardar uma textura exigia manter o
  loader vivo por todo o programa; agora basta a janela.
- `ResourceManager<'w>` cobre texturas, sons e músicas. Fontes ficam em `FontCache<'t>` porque
  `Font<'t, 'static>` empresta o `Sdl2TtfContext` de um `FontLoader` concreto — misturar os dois
  lifetimes obrigaria a `Engine` a guardar o `FontLoader`, recriando o problema de auto-referência.
- `Scene` mantém métodos sem parâmetros genéricos, então `Box<dyn Scene + 'w>` aceita cenas
  heterogêneas. `on_enter` recebe `SceneContext<'_, 'w>` com `&mut ResourceManager<'w>` e áudio
  opcional; `update`/`render` recebem apenas o que precisam, e a cena guarda `Rc<Texture<'w>>`
  obtido na entrada.
- Transições de cena são aplicadas entre quadros. Uma troca imediata durante `update` exigiria
  `&mut SceneManager` com a cena já emprestada pela pilha (inviável sem `Rc<RefCell<...>>` ou
  comandos). Adiar a transição também evita trocar a cena no meio da atualização. O efeito — a cena
  nova desenha no quadro seguinte — está documentado e coberto por teste.
- `Engine::step(dt)` existe para testes headless e para jogos que querem controlar o ritmo
  (o `mini_game` troca cenas pelo `scenes_mut()` entre passos). `Engine::run` usa
  `game_loop::FixedStepper`, exportado como peça reutilizável do loop.
- `RenderContext::ensure_cleared` limpa o quadro apenas se a cena não limpou, garantindo fundo
  definido sem duplo `clear` em todo quadro.
- O áudio é opcional na `Engine`: falha ao abrir o dispositivo vira log e `audio() == None`, para
  que jogos funcionem em imagens sem backend de som (caso real do GIOCHI KONSOL OS).

## Assets e áudio

- `ResourceManager` guarda recursos em `Rc` para permitir que sprite, cena e cache compartilhem a
  mesma textura sem recarregar o arquivo. Cada requisição incrementa `hit` ou `miss`; `loads()`
  expõe apenas as cargas reais, que é o critério usado nos testes (duas chamadas, um load).
- O cache não faz `Drop` dos recursos SDL: texturas e fontes têm lifetime ligado ao
  `TextureLoader`/`FontLoader` da janela. Por isso o cache deve ser declarado depois do loader,
  garantindo que seja liberado antes dele.
- `loader` diferencia caminho inválido (`InvalidPath`) de falha de leitura (`Io`) e de
  decodificação (`Texture`, `Audio`, `Font`), para que a aplicação possa reagir a recurso ausente
  sem tratar como erro fatal de backend.
- `Sound` aceita arquivo ou amostras PCM de 16 bits (`from_samples`) e tem `tone()` para síntese
  simples; isso permite efeitos procedurais e testes sem assets de áudio versionados.
- `MusicTrack` encapsula `sdl2::mixer::Music` e expõe `play`, `play_loop`, `pause`, `resume`,
  `stop` e volume. Pausa/retomada são globais no SDL_mixer e afetam a música corrente, o que está
  documentado na API.
- `AudioSystem` abre o dispositivo no `init` e fecha no `Drop`; `allocate_channels` só cresce, nunca
  reduz, para não interromper canais em uso. Falta de dispositivo é erro tipado
  (`AudioError::DeviceOpen`), e o exemplo cai para o driver `dummy` quando a imagem não tem backend
  de áudio.
- Fontes ficam no `assets` porque o acesso ao arquivo e o cache são responsabilidade de recursos;
  o desenho de texto será adicionado junto de UI/`ttf` em fase posterior.

## Estado da implementação

- Implementados: inicialização SDL2, janela/canvas, loop fixo, renderização de primitivas e
  texturas, sprites, câmera mundo/tela, entrada de teclado/mouse/gamepad e estruturas básicas de
  matemática/cache/estado.
- Implementados: plataforma, entrada, renderização, assets, áudio, cenas (`Scene`,
  `SceneContext`, `SceneManager`) e a fachada `Engine`/`EngineError`. Restam polimento, release e
  documentação de API (Fase 6).
- Erros de biblioteca usam `thiserror`; aplicações podem agregar contexto com `anyhow`.
- `thiserror` fornece erros tipados e sem dependência de contexto de aplicação na biblioteca;
  `anyhow` facilita a composição e apresentação de erros no binário e nos exemplos.
- O binário principal permanece uma demonstração SDL2 mínima de janela preta e saída por Escape;
  os exemplos independentes demonstram retângulo animado, sprite controlado pela entrada e o
  agregado `Input` com teclado, mouse e gamepad.
