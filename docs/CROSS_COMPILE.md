# Cross-compilation para ARM64

## Instalação

Instale Docker, QEMU/binfmt para execução de ferramentas ARM durante a preparação do container e
Rust stable. Os workflows configuram a emulação ARM64 automaticamente. O arquivo
`rust-toolchain.toml` declara o target; também é possível instalá-lo manualmente:

```sh
rustup target add aarch64-unknown-linux-gnu
cargo install cross --locked
```

Em uma máquina local, configure o Docker para emular ARM64 (por exemplo, com `qemu-user-static`
e binfmt_misc) antes de usar um `Cross.toml` que instala pacotes multiarch. Sem essa emulação,
scripts `postinst` de pacotes ARM podem falhar durante a criação do container.

## Configuração e build

`Cross.toml` seleciona a imagem `ghcr.io/cross-rs/aarch64-unknown-linux-gnu:main` e instala
dependências de desenvolvimento SDL2, DRM, GBM, EGL/GLES, udev e ALSA na imagem de compilação.
O target padrão do Cross é ARM64. Compile explicitamente:

```sh
cross build --target aarch64-unknown-linux-gnu --release
```

O executável será `target/aarch64-unknown-linux-gnu/release/giochi-demo`. Para testar a cadeia
sem criar release, use o workflow manual `Cross Compile`.

## Transferir para o dispositivo

Com SSH configurado:

```sh
scp target/aarch64-unknown-linux-gnu/release/giochi-demo usuario@dispositivo:/tmp/
ssh usuario@dispositivo 'chmod +x /tmp/giochi-demo && /tmp/giochi-demo'
```

Alternativamente, monte uma pasta via SSHFS:

```sh
mkdir -p /tmp/giochi-device
sshfs usuario@dispositivo:/home/usuario /tmp/giochi-device
cp target/aarch64-unknown-linux-gnu/release/giochi-demo /tmp/giochi-device/
fusermount -u /tmp/giochi-device
```

## Dependências de runtime

No dispositivo, verifique a disponibilidade e compatibilidade de:

- SDL2 e extensões SDL2_image, SDL2_mixer e SDL2_ttf;
- DRM, GBM, EGL e OpenGL ES;
- udev e ALSA, se os recursos correspondentes forem usados;
- bibliotecas padrão do sistema do target, incluindo glibc compatível;
- permissões de execução em `/dev/dri` e `/dev/input`.

As extensões SDL2 são ligadas ao sistema; a opção `bundled` compila apenas o SDL2 principal.
Distribuições alvo devem oferecer as bibliotecas SDL extension compatíveis.
