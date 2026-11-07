# SDL2 com KMSDRM

## O que é KMSDRM

KMS (Kernel Mode Setting) configura modos de vídeo diretamente pelo kernel Linux; DRM
(Direct Rendering Manager) fornece a interface de acesso à GPU. O driver KMSDRM do SDL2 permite
desenhar em um console sem servidor X11 ou Wayland, reduzindo camadas em dispositivos embarcados.
O usuário precisa de acesso autorizado aos dispositivos DRM e de entrada.

## Variáveis de ambiente

| Variável | Uso |
|---|---|
| `SDL_VIDEODRIVER` | Define o driver; `KMSDRM` seleciona o console, `x11` ou `wayland` permite desktop. O exemplo SDL2 usa KMSDRM apenas se não houver definição anterior. |
| `SDL_VIDEO_EGL_DRIVER` | Caminho/driver EGL utilizado pelo SDL2, conforme a distribuição. |
| `SDL_VIDEO_GL_DRIVER` | Biblioteca OpenGL a carregar quando necessário. |
| `SDL_KMSDRM_DEVICE_INDEX` | Seleciona o dispositivo DRM quando há mais de uma GPU. |
| `SDL_KMSDRM_REQUIRE_DRM_MASTER` | Controla a exigência de DRM master pelo backend KMSDRM. |

Execute a partir de uma TTY, encerre o compositor gráfico se ele mantiver a posse do dispositivo
DRM e use permissões/grupos apropriados:

```sh
sudo usermod -aG input,video "$USER"
export SDL_VIDEODRIVER=KMSDRM
cargo run --release --example moving_sprite
```

Faça logout/login após alterar grupos. Não execute como root como solução permanente.

## Diagnóstico

- **Tela preta:** confirme que a saída está conectada e suportada, rode em TTY, verifique logs
  com `RUST_LOG=debug` e tente outro modo de vídeo disponível no dispositivo.
- **Falha de EGL:** confira os pacotes/drivers EGL e GLES correspondentes à GPU; remova overrides
  `SDL_VIDEO_EGL_DRIVER`/`SDL_VIDEO_GL_DRIVER` inválidos e teste a versão do SDL2 do sistema.
- **Permissão negada em `/dev/input`:** confira associação ao grupo `input`, regras udev e
  permissões do dispositivo; refaça o login após alterar grupos.
- **DRM ocupado:** encerre a sessão gráfica que controla DRM master e verifique
  `/dev/dri/card*` e o índice escolhido.

## Plataformas para teste

- **Raspberry Pi OS:** use uma imagem atual com KMS habilitado e pacotes SDL2/EGL da distribuição;
  teste em TTY no usuário do dispositivo.
- **Anbernic:** o suporte varia conforme modelo e firmware. Confirme kernel DRM, acesso a `/dev/dri`
  e bibliotecas SDL/EGL fornecidas pela imagem antes de validar o demo.
- **Arch ARM:** instale `sdl2`, `sdl2_image`, `sdl2_mixer`, `sdl2_ttf`, `mesa` e `libdrm`;
  confirme os grupos e teste a partir de um console local.

As distribuições e firmwares embarcados têm versões e configurações diferentes; valide cada
combinação de placa, kernel e driver em hardware real.

## Resultado validado em hardware (R35S / RK3326)

Teste executado em um GIOCHI KONSOL OS (Buildroot 2024.02, kernel 6.12, glibc 2.38) com
`SDL_VIDEODRIVER=kmsdrm`:

- O driver relatado pelo SDL2 foi `KMSDRM` e o dispositivo abriu `/dev/dri/card0` sem X11/Wayland.
- O gamepad embutido foi detectado como `r36s_Gamepad` pelo subsistema de controle do SDL2.
- A captura via `RenderContext::read_pixels` confirmou o quadro: fundo `(20,60,140)` e retângulo
  `(240,90,40)` de 120x120 px na posição esperada, sem deslocamento de cor ou escala.
- `hello_rect`, `moving_sprite` e `input_test` permaneceram ativos durante a execução em KMSDRM.

Observações específicas dessa imagem:

- `/dev/fb0` (`rockchipdrmfb`) **não** espelha o scanout do KMSDRM; usar `read_pixels` para
  evidência visual em vez de ler o framebuffer legado.
- O SDL2 dessa imagem foi compilado sem backend ALSA (apenas `dsp`, `dummy` e `disk`). Para testar
  áudio sem hardware use `SDL_AUDIODRIVER=dummy`; para áudio real, habilite ALSA no Buildroot.
- O SDL2_image foi compilado sem PNG. A engine inicializa o SDL_image sob demanda, então builds
  parciais não impedem a inicialização; o erro aparece apenas ao carregar um PNG.
