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

## Build contra o sysroot do dispositivo (validado em R35S)

Quando a imagem Buildroot já entrega SDL2 com KMSDRM habilitado, compilar contra as bibliotecas do
próprio aparelho evita incompatibilidade de ABI e dispensa recompilar o SDL2. O resultado depende
apenas do `libc` e das extensões SDL já presentes no rootfs.

Ative a feature `sdl2-system`, que troca `bundled` por `use-pkgconfig`, e aponte o `pkg-config`
para um sysroot montado a partir do dispositivo:

```sh
# 1. Monte um sysroot mínimo com as bibliotecas do aparelho
mkdir -p /tmp/r35s-sysroot/usr/{lib/pkgconfig,include/SDL2}
scp root@10.55.0.2:/usr/lib/libSDL2-2.0.so.0.2800.5 \
    root@10.55.0.2:/usr/lib/libSDL2_image-2.0.so.0.800.2 \
    root@10.55.0.2:/usr/lib/libSDL2_ttf-2.0.so.0.2200.0 \
    /tmp/r35s-sysroot/usr/lib/
cd /tmp/r35s-sysroot/usr/lib
ln -sf libSDL2-2.0.so.0.2800.5 libSDL2.so
ln -sf libSDL2_image-2.0.so.0.800.2 libSDL2_image.so
ln -sf libSDL2_ttf-2.0.so.0.2200.0 libSDL2_ttf.so
```

Crie `sdl2.pc`, `SDL2_image.pc`, `SDL2_ttf.pc` e `SDL2_mixer.pc` em
`/tmp/r35s-sysroot/usr/lib/pkgconfig` apontando `libdir` para esse diretório e `includedir` para
os headers SDL (os headers são apenas informativos; o `sdl2-sys` usa bindings Rust).

Compile:

```sh
PKG_CONFIG_ALLOW_CROSS=1 \
PKG_CONFIG_PATH=/tmp/r35s-sysroot/usr/lib/pkgconfig \
cargo build --release --target aarch64-unknown-linux-gnu \
      --no-default-features --features sdl2-system
```

O `.cargo/config.toml` já define `linker = "aarch64-linux-gnu-gcc"` para o target. Confira a ABI
antes de copiar:

```sh
aarch64-linux-gnu-objdump -T target/aarch64-unknown-linux-gnu/release/giochi-demo \
  | grep -o 'GLIBC_2\.[0-9]*' | sort -u -V | tail -1
```

A versão máxima listada precisa ser menor ou igual à do `libc` do aparelho (`GLIBC_2.38` no
R35S com Buildroot 2024.02).

### SDL2_mixer ausente na imagem

Se `ldd`/`readelf` apontar `libSDL2_mixer-2.0.so.0` e a imagem não a tiver, compile-a a partir do
código-fonte com os codecs mínimos e instale no aparelho:

```sh
curl -LO https://www.libsdl.org/projects/SDL_mixer/release/SDL2_mixer-2.8.0.tar.gz
tar xzf SDL2_mixer-2.8.0.tar.gz && cd SDL2_mixer-2.8.0
PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH=/tmp/r35s-sysroot/usr/lib/pkgconfig \
./configure --host=aarch64-linux-gnu --prefix=/tmp/r35s-sysroot/usr --disable-sdltest \
  --disable-music-flac --disable-music-mp3 --disable-music-mod --disable-music-ogg \
  --disable-music-midi --disable-music-opus --disable-music-wavpack \
  CFLAGS="-O2 -I/tmp/r35s-sysroot/usr/include/SDL2" \
  LDFLAGS="-L/tmp/r35s-sysroot/usr/lib"
make -j"$(nproc)" && make install
scp /tmp/r35s-sysroot/usr/lib/libSDL2_mixer-2.0.so.0.800.0 root@10.55.0.2:/usr/lib/
ssh root@10.55.0.2 'cd /usr/lib &&
  ln -sf libSDL2_mixer-2.0.so.0.800.0 libSDL2_mixer-2.0.so.0 &&
  ln -sf libSDL2_mixer-2.0.so.0 libSDL2_mixer.so'
```

Com os codecs desabilitados a biblioteca depende apenas de `libSDL2` e `libc`, exigindo
`GLIBC_2.17`. O suporte a WAV continua embutido.

## Execução no aparelho

```sh
scp target/aarch64-unknown-linux-gnu/release/giochi-demo root@10.55.0.2:/tmp/
ssh root@10.55.0.2 'chmod +x /tmp/giochi-demo &&
  SDL_VIDEODRIVER=kmsdrm RUST_LOG=info /tmp/giochi-demo'
```

Se outro processo detiver o DRM master (launcher do console, por exemplo), o KMSDRM falha ao
configurar o modo. Libere o dispositivo antes do teste e restaure o serviço depois.
