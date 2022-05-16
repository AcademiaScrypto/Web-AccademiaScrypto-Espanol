# 💾 Instalación simulador

::: tip 📺 Vídeo
 - [Instalación Windows](https://youtu.be/HmX4-yQGLmA)
:::

::: warning Documentación:
- Actualmente nos encontramos en la versión [0.4](https://github.com/radixdlt/radixdlt-scrypto/releases/tag/v0.4.0) de Scrypto.
- [Notas de lanzamiento](https://docs.radixdlt.com/main/scrypto/release_notes/v0_4.html)
- [Anuncio del lanzamiento](https://www.radixdlt.com/post/scrypto-v0-4-released)
- [Migración de la versión 0.3 a la versión 0.4](https://docs.radixdlt.com/main/scrypto/release_notes/migrating_from_0.3_to_0.4.html) ([Actualización](../instalacion/actualizacion.md))
:::

Vamos a listar el software que necesitas para poder empezar:
1. Un editor de código: Nuestra recomendación es usar [Visual Studio Code](https://code.visualstudio.com/) por varios motivos, pero el más importante es que es gratuito y multiplataforma lo que quiere decir que puedes instalarlo en Windows, Linux o macOS. *Es el que utilizaremos durante el curso.*
2. Instalar git, que es una herramienta de versionado.
    - Windows:
        Descargar e instalar: [Git](https://git-scm.com/download/win)
    - Linux y macOS
        Instalar C++ y Git:  
        1.
        ```
        sudo apt install build-essential
        ```
        2. Seguir los pasos: [Git](https://git-scm.com/book/es/v2/Inicio---Sobre-el-Control-de-Versiones-Instalaci%C3%B3n-de-Git)
2. Instalaremos Rust y herramientas LLVM:
    - Windows: 
        - Instalaremos "Desktop development with C++" que se encuentra dentro de [Build Tools for Visual Studio 2019](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16)
        - Descargaremos e instalamos: [rustup-init.exe](https://rustup.rs/)
        - Instalar [LLVM 13.0.1](https://github.com/llvm/llvm-project/releases/download/llvmorg-13.0.1/LLVM-13.0.1-win64.exe) *Importante:  Asegurate de marcar la opción que agrega LLVM a la **ruta** del sistema*
    - Linux y macOs:
    ```
       apt install llvm clang
       curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```
2. Habilitamos `cargo` desde la linea de comando:
   * Windows:
       * Iniciamos Windows PowerShell: 
       ```
       set path=%USERPROFILE%/.cargo/env
       ```
   * Linux and macOS:
       ```
       source $HOME/.cargo/env
       ```
3. Añadimos WebAssembly target desde la misma linea de comando:
    ```
    rustup target add wasm32-unknown-unknown
    ```
4. Instalamos el simulador:
    ```
   git clone https://github.com/radixdlt/radixdlt-scrypto
    cd radixdlt-scrypto
    cargo install --path ./simulator
    ```
4. Comprobación de instalacion OK:
    Dentro de la carpeta radixdlt-scrypto escribimos desde la linea de comando:
    ```
   scrypto new-package tutorial
    ```
    si no aparece ningún mensaje es que todo fue correcto. A continuación escribimos en la linea de comandos:
    ```
    resim new-account
    ```
    si ves el mensaje: "Transaction Status: SUCCESS " ya tienes todo configurado correctamente. 
    ![Success instalation](/success.png)


::: tip
- Si has tenido algún problema al instalar el simulador, asegurate de ejecutar la última version de Rust.
```
rustup update stable
```
:::

Si todo ha salido bien ya tienes en tu computadora un simulador de la red Radix junto con el lenguaje Scrypto y su compilador para que puedas realizar todo tipo de pruebas. ¿Te sientes con ganas de empezar ya? te dejo un enlace al video de la presentación a cargo del CTO Russell Harvey [Presentación de Scrypto](https://www.youtube.com/watch?v=he9TunEXgcY)


### Bibliografia:
- [https://github.com/radixdlt/radixdlt-scrypto](https://github.com/radixdlt/radixdlt-scrypto)