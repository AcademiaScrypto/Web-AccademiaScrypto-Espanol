# ¡Hola Mundo!

::: warning
- Esta unidad está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Video ¡Hola Mundo!]()
:::

Aunque todavía son muchos los conceptos básicos de programación [(Rust)](../../rust/README.md) y específicos de Scrypto que no hemos visto. Vamos a echar un primer vistazo al código de un *Componente*, que es como se conoce a los contratos inteligentes en Scrypto.  

Para ello vamos a utilizar el edito de código que instalamos en la [Unidad 1](/scrypto/instalacion/instalacion.md), [Visual Studio Code](https://code.visualstudio.com/) (VSC). *Nota: Puedes utilizar el editor que prefieras*.

### Pasos inicio con VSC:
 - Abrimos el Visual Studio Code (VSC)
 - Archivo -> Abrir carpeta
 - Buscamos y seleccionamos la carpeta donde instalamos el simulado de Scrypto: *En windows: C:\Users\TU_USUARIO\radixdlt-scrypto*
    - Puede ser que nos pregunte si confiamos en los autores, marcaremos que Sí.
-  Abriremos el terminal desde el menu -> Ver -> Terminal 

::: tip
Para abrir el terminal desde un VSC (instalado en windows) podemos utilizar **atajos de teclado**, en este caso **Ctrl+Ñ**.  

Recuerda los atajos de teclado pueden agilizar mucho tu trabajo de desarrollo.

- OJO: ten muy presente que el terminal no tiene una correspondencia con el explorador de archivos de VSC.
:::

### Crear nuevo BluePrint
![BluePrint](/blueprint.png)
::: tip
- Un blueprint, en Radix, es como un plano que define la estructura y la lógica de un *Componente* o contrato inteligente. 
:::

El comando para crear un BluePrint es el siguiente:
```
scrypto new-package <nombre>
```

Donde *nombre* puede se lo que queramos, por ejemplo:
```
scrypto new-package miprimerBluePrint
```

### ¡Pruébalo!

Escribe en tu terminal desde VSC el comando para la creación de un BluePrint:
```
scrypto new-package miprimerBluePrint
```
![nuevo blueprint](/new_blue.png)  

*Nota: Si no ha dado ningún mensaje por terminal es que todo ha salido bien*

¿Qué ha sucedido? Se ha creado una nueva carpeta con el nombre de tu componente y dentro de esta dos carpetas, de momento nos vamos a centrar en la carpeta *src* que contiene un archivo llamado *lib.rs* que es donde está la lógica de tu blueprint.  

![mi primer blueprint](/miprimerblue.png)  

::: tip
La extensión de los archivos en Scrypto será *.rs*, que es la nativa de Rust.
:::

### Análisis del código linea a linea de *lib.rs*:
<br>
<div class="alert alert-success" role="alert">
 No te asustes... ¡es solo código!, no tienes que entenderlo todo ahora, ¡pronto lo harás!
</div>


La primera línea, no tenemos que entenderla ahora, solo explicaros que es la que **une Rust con Scrypto**.  

```rust
use scrypto::prelude::*;
```

Seguidamente nos encontramos con esta sentencia **blueprint!**  

```rust
blueprint! {
    ...código
}
```
Indica que estamos escribiendo un BluePrint en su interior está el código que definirá todas las instrucciones que contendrá el blueprint.

::: tip
    ¡Te diste cuenta! en el código se abren y cierran corchetes {...} así como paréntesis (...), así es como se acotan los diferente partes/elementos del código en Rust.
:::

Básicamente nuestro blueprint, o plano con el que luego construiremos/instanciaremos componentes, contendrá dos elementos principales: las estructura (*struct*) y la implementación de la estructura (*impl*):  

```rust
blueprint! {
    struct <nombre> {
        ...Definición
    }
    impl <nombre> {
        ...Funciones
    }
}
```
Dentro de la estructura, **struct**, definiremos los recursos y datos que posteriormente serán administrados por el componente creado a partir de un blueprint.

Dentro de la implementación, **impl**, se crearan todas las funciones (fn) y métodos necesarias para administraran los recursos y datos creados en la estructura (*struct*)

### La Estructura (**struct**)

En el ejemplo que se ha creado automáticamente al generar un *new-package*, llamado *Hello*, la estructura define un **Vault** o contenedor de recursos **permanente** con el nombre *sample_vault*:  

```rust
struct Hello {
        sample_vault: Vault
    }
```
*Nota: en la unidad siguiente veremos más acerca de la diferencia entre un contenedor **Vault** y **Bucket**.*

### La Implementación (**impl**)

En el ejemplo que estamos analizando, implementaremos la estructura *Hello*. Dentro de esta implementación se crean dos funciones (fn) públicas (pub), una llamada instantiate_hello() y otra llamada free_token(&mut self). Esta segunda función, *free_token*, está recibiendo un parámetro de entrada *(&mut self)*, en este caso es una referencia mutable a si mismo, con lo que esta función se convierte en un método.  
*Nota: Ya veremos más adelante la diferencia en Rust entre funciones y métodos.*

::: tip
Recuerda: [las variables en Rust](/rust/variables.md) por definición son inmutables. Pero con la palabra clave *mut* podemos invertir el estado de la variable a mutable.
:::

```rust
impl Hello {

        pub fn instantiate_hello() -> ComponentAddress {
            
            let my_bucket: Bucket = ResourceBuilder::new_fungible()
                .metadata("name", "HelloToken")
                .metadata("symbol", "HT")
                .initial_supply(1000);

            Self {
                sample_vault: Vault::with_bucket(my_bucket)
            }
            .instantiate()
            .globalize()
        }

        pub fn free_token(&mut self) -> Bucket {
            info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
            self.sample_vault.take(1)
        }
    }
```

La primera función, **instantiate_hello()**, crea un *Componente* que contendrá un token fungible de nombre "HelloToken", símbolo "HT y supply 1000. Este nuevo componente recibirá el nombre *Hello*.  

La segunda función, **free_token(&mut self)**, que es un **método** que tomará 1 *HelloToken* con la siguiente instrucción:

```rust
self.sample_vault.take(1)
```

Es importante hacer notar lo expresivo que es Scrypto: **take** es una método que toma un recurso de un contenedor y lo devuelve, en este caso de un contenedor permanente **Vault** definido en la estructura.

::: tip
    **self** esta haciendo referencia a la estructura *Hello* que estamos implementando.
:::

Finalmente devolverá el mensaje: *"My balance is: 1 HelloToken. Now giving away a token!"* con la siguiente instrucción:

```rust
info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
```

<div class="alert alert-success" role="warning">
 ¡FELICIDADES!, ya has leído tu primer Contrato Inteligente de Scrypto 🥳🎉🎉🎉🎉
</div>

::: warning Importante:
Soy muy consciente que hay muchas que no has entendido, ¡TRANQUILO!, las entenderás...
:::