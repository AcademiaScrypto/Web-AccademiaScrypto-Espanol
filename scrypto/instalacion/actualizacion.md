# 🆕 Actualización Simulador

::: tip Vídeo guía
- [Youtube](https://youtu.be/r00ED28ejAI)
:::

Hasta llegar a Babilonia a principios del 2023 Scrypto sufrirá cambios con el fin de mejorar y corregir errores. Actualmente con el lanzamiento de Alexandria Scrypto se encuentra en la versión [0.4](https://github.com/radixdlt/radixdlt-scrypto/releases/tag/v0.4.0) y la idea del equipo de desarrolladores de Radix es que en Babilonia se pueda presentar la versión 1.0. A continuación vamos a enumerar los pasos a seguir para actualizar nuestro simulador de Scrypto:

::: warning Documentación v0.4:
- [Notas de lanzamiento](https://docs.radixdlt.com/main/scrypto/release_notes/v0_4.html)
- [Anuncio del lanzamiento](https://www.radixdlt.com/post/scrypto-v0-4-released)
- [Migración de la versión 0.3 a la versión 0.4](https://docs.radixdlt.com/main/scrypto/release_notes/migrating_from_0.3_to_0.4.html)
:::

::: tip Atención!!!!
- Debemos situarnos en la carpeta **radixdlt-scrypto**
:::

1. Actualizaremos Rust e instalar LLVM y Clang *(si fuera necesario)*:
    - Windows:
        ```
        rustup update stable
        ```
        *Este paso solo lo realizaremos si venimos de una versión previa a la 0.3 de Scrypto*
        - Instalar [LLVM 13.0.1](https://github.com/llvm/llvm-project/releases/download/llvmorg-13.0.1/LLVM-13.0.1-win64.exe) *Importante:  Asegurate de marcar la opción que agrega LLVM a la **ruta** del sistema*
    - Linux:
        ```
        rustup update stable
        // El siguiente paso solo lo realizaremos si venimos de una versión previa a la 0.3 de Scrypto
        apt install llvm clang
        ```


2. Vamos a actualizar nuestro simulador con los cambios ya publicados en Github, para ello entraremos en la carpeta **radixdlt-scrypto** y lanzaremos el siguiente comando de Github que actualiza solo los archivos modificados:

```
git pull https://github.com/radixdlt/radixdlt-scrypto.git
```

3. Instalamos el Simulador:
```
cargo install --path ./simulator
```

4. Limpiamos los datos del Simulador:
```
resim reset
```

4. Probar el Simulador por ejemplo creando una cuenta o creando un Package
```
¡Eso ya sabes hacerlo! 🤭
```
::: details Pero... por si eres un 'Homo Emilius' te dejo una ayudita 🙈.
1. Limpiar el simulador
```rust
resim reset
```
2. Crear un Package
```rust
scrypto new-package ejemplo4
cd ejemplo4
```
3. Crear una cuenta
```rust
resim new-account
```
:::

5. Opcional, instalar Documentación:
```
./doc.sh
```
(*Nota: la documentación también la tienes en linea: [The Scrypto Standard Library](https://radixdlt.github.io/radixdlt-scrypto/scrypto/index.html)*)

### Bibliografia
- [Updating Scrypto to the latest version](https://docs.radixdlt.com/main/scrypto/getting-started/updating-scrypto.html)