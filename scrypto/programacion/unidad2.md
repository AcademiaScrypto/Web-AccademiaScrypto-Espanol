# Simulador: Hola Mundo! en acción

::: warning
- Esta unidad está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Soon Video]()
:::

### Instalar extension Rust para VSC (Visual Studio Code)

Las Extensiones, en VSC, permiten ampliar las funciones y herramientas disponibles. Hoy recomendamos la extensión de Rust para tu mejor desempeño de la codificación de los 'BluePrints'.
- [Rust extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)


### Crear nueva cuenta

::: tip
- Acostumbrate antes de empezar a trabajar con Scrypto a limpiar el simulador. En realidad muchas veces limpiar el simulador de datos nos permitirá resolver algunos problemas de ejecución. Realizaremos dicha limpieza ejecutando desde la linea de comandos (terminal):
```
resim reset
```
:::

Para interactuar con el simulador, Radix ha creado un comando llamado [**resim**](/scrypto/conceptos/conceptos3.md#resim). Con el comando *resim* de forma básica podemos crear cuentas para poder trabajar con ellas, estas cuentas nos permitirán entre otras cosas guardar recursos e interactuar con los *componentes*. 

El sub-comando para crear una nueva cuenta es **new-account**:
```
PS C:\Users\<mi_usuario>\radixdlt-scrypto> resim new-account
Account component address: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
Public key: 044083a64afb4b630ce7683674a6cdcebc7007aef7cb08f10b2cd491b6ce24ca1204f88bd2a2068e27591f1c5cfbd4fddf9a51f7b2360d784ee1e8fbec8f7476a6
Private key: 7c9fa136d4413fa6173637e883b6998d32e1d675f88cddff9dcbcf331820f4b8
No configuration found on system. will use the above account as default.
```
Este es el resultado de ejecutar el comando *resim new-account* desde el terminal.  

Nos fijamos en especial en: **Account component address**
```  
Account component address: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
```

Esta dirección nos va permitir utilizar la nueva cuenta.

::: tip
- Si quieres ver el contenido de esta cuenta utiliza el comando: *resim show [account address]*
:::

Podemos guardar la dirección, y cualquier otro dato, dentro de variables de sistema para que nos sea más fácil escribir los comandos:  

Windows:
```rust
set account [tu numero de cuenta]
// ejemplo: set account 02b8a2383a7d462575e673153ae12e5ed78ee5142abe2a8abcab58 (seguido de un intro)
```
Linux/macOs:
```rust
export account=[tu numero de cuenta]
// ejemplo: export account=02b8a2383a7d462575e673153ae12e5ed78ee5142abe2a8abcab58 (seguido de un intro)
```
*(Nota: recuerda como se guardan las variables de sistema en linux: export nombre=valor, no vamos a repetirlo!)*

Para reutilizar estas variables de sistema lo haremos utilizando el símbolo clave **$** justo delante del nombre de la variable: (ej. resim show $account) 
```rust
PS C:\Users\<mi_usuario>\radixdlt-scrypto>resim show $account
Component: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
Blueprint: { package_address: 010000000000000000000000000000000000000000000000000003, blueprint_name: "Account" }
Authorization
├─ "deposit" => AllowAll
└─ "deposit_batch" => AllowAll
State: Struct(LazyMap("bc417218214859fbbf019072394c50cc53d5419f4acd7a660dc7c880f0cce31a02040000"))
Lazy Map: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a(bc417218214859fbbf019072394c50cc53d5419f4acd7a660dc7c880f0cce31a, 1026)
└─ ResourceAddress("030000000000000000000000000000000000000000000000000004") => Vault("bc417218214859fbbf019072394c50cc53d5419f4acd7a660dc7c880f0cce31a03040000")
Resources:
└─ { amount: 1000000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
``` 

::: tip
- ¡Posiblemente te diste cuenta!, con la creación de una nueva cuenta se nos incluye como recurso (resource) unos cuantos XRD, ¡de mentiras 😅! 
:::

Una vez tenemos una cuenta, vamos a publicar el *Package/Blueprint* que creamos en la [unidad anterior](/scrypto/programacion/unidad1.md). Si seguiste los pasos, tu *blueprint* se encuentra dentro de la carpeta: *miprimerBluePrint*. Para acceder a la carpeta no olvides utilizar el comando **cd** seguido del nombre de la carpeta.  

```
PS C:\Users\<mi_usuario>\radixdlt-scrypto> cd miprimerblueprint
```
Dentro del directorio que alberga nuestro *blueprint* en el terminal vamos a proceder a publicar el blueprint en el simulador. Este acto (publicar) lo que hace, explicado de forma muy burda, es traducir nuestro código a un idioma que entienda la red de Radix. Ojo que este paso no ha creado ningún *Componente* o contrato inteligente, solo ha publicado los planos de nuestro futuro componente dentro de la red.  

El comando para publicar el *blueprint* es: **resim publish .** *(fíjate que entre publish y punto hay un espacio!!!)*
```
PS C:\Users\<mi_usuario>\radixdlt-scrypto\miprimerblueprint> resim publish .
    Updating crates.io index
  Downloaded ryu v1.0.6
  Downloaded serde_json v1.0.72
  Downloaded syn v1.0.82
  Downloaded 3 crates (400.2 KB) in 2.35s
   Compiling proc-macro2 v1.0.32
   Compiling unicode-xid v0.2.2
            [...]
   Compiling scrypto v0.1.0 (C:\Users\IEUser\radixdlt-scrypto\scrypto)
   Compiling miprimerBluePrint v0.1.0 (C:\Users\IEUser\radixdlt-scrypto\miprimerblueprint)
    Compiling sha2 v0.9.9
   Compiling sbor-derive v0.3.0 (https://github.com/radixdlt/radixdlt-scrypto?tag=v0.3.0#7cb4af0b)
   Compiling sbor v0.3.0 (https://github.com/radixdlt/radixdlt-scrypto?tag=v0.3.0#7cb4af0b)
   Compiling scrypto-abi v0.3.0 (https://github.com/radixdlt/radixdlt-scrypto?tag=v0.3.0#7cb4af0b)
   Compiling scrypto-derive v0.3.0 (https://github.com/radixdlt/radixdlt-scrypto?tag=v0.3.0#7cb4af0b)
   Compiling scrypto v0.3.0 (https://github.com/radixdlt/radixdlt-scrypto?tag=v0.3.0#7cb4af0b)
   Compiling tutorial v0.1.0 (C:\Users\Andres\radixdlt-scrypto\tutorial)
    Finished release [optimized] target(s) in 37.20s
Success! New Package: 018e40a0a6dd954f296a26a0d849c7f1262bb8f9b61987c5f23d33
```
La red de Radix asigna una dirección al *Package/Blueprint*, la encontramos en la última línea bajo **Package**:
```
Success! New Package: 018e40a0a6dd954f296a26a0d849c7f1262bb8f9b61987c5f23d33
```
Guardaremos esta dirección en una variable de sistema, tal y como hicimos antes con account:

Windows:
```
set package [tu dirección del package]
//ejemplo: set package 018e40a0a6dd954f296a26a0d849c7f1262bb8f9b61987c5f23d33
```  

Linux/macOs:
```
export package=[tu dirección del package]
//ejemplo: export package=018e40a0a6dd954f296a26a0d849c7f1262bb8f9b61987c5f23d33
```

Ahora ya estamos en disposición de crear nuestro primer componente 👏👏👏, si has llegado hasta aquí eres un/a VALIENTE!!!   

Recordaras en [Scrypto: Conceptos 1](/scrypto/conceptos/conceptos1.md) ya vimos que un *blueprint* no era mas que los planos del *componente*, ahora que ya tenemos el 'plano' miprimerblueprint implementado en el libro mayor del Simulador, ahora ya podemos llamar a su función *constructora* la cual creara (instanciara) un nuevo componente a partir de los planos.   
La estructura de la instrucción es la siguiente: **resim call-function [package] [blueprint] [función]**

```rust
resim call-function $package Hello instantiate_hello
```


El resultado:
```rust
Transaction Status: SUCCESS
Execution Time: 20 ms
Instructions:
├─ CallFunction { package_address: 018e40a0a6dd954f296a26a0d849c7f1262bb8f9b61987c5f23d33, blueprint_name: "Hello", function: "instantiate_hello", args: [] }
└─ CallMethodWithAllResources { component_address: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a, method: "deposit_batch" }
Instruction Outputs:
├─ ComponentAddress("02fb3252c37ccf33f54b447f6a4f74580b36f375b6c232a5f7a8bd")
└─ ()
Logs: 0
New Entities: 2
└─ Component: 02fb3252c37ccf33f54b447f6a4f74580b36f375b6c232a5f7a8bd
└─ Resource: 0394a0e64bbd357867d9e4904a7011b9977fcbb01c87c80a908896
```

En esta ocasión al llamar a la función *instantiate_hello* ha dado como resultado dos nuevas entidades en el libro mayor del Simulador:
- **Resource:** que hace referencia a un nuevo recurso creado, un token llamado 'HelloToken'
- **Component:** que hace referencia al nuevo *Componente* 'Hello'
```
New Entities: 2
└─ Component: 02fb3252c37ccf33f54b447f6a4f74580b36f375b6c232a5f7a8bd
└─ Resource: 0394a0e64bbd357867d9e4904a7011b9977fcbb01c87c80a908896
```
Guardaremos la dirección del componente en una variable de sistema, tal y como hicimos antes con package:
```
set component [tu dirección del componente]
//ejemplo: set component 02fb3252c37ccf33f54b447f6a4f74580b36f375b6c232a5f7a8bd
```

Ya tenemos un **Componente** o contrato inteligente viviendo en el Simulador de Radix, ya solo nos queda comprobar que funciona, para ello vamos a llamar al método que tiene, ¿recuerdas de la lección pasada? *free_token* (*Nota:seria bueno revisar el código de este Componente para refrescar la memoria)*:
```rust
pub fn free_token(&mut self) -> Bucket {
            info!("My balance is: {} HelloToken. Now giving away a token!", self.sample_vault.amount());
            // If the semi-colon is omitted on the last line, the last value seen is automatically returned
            // In this case, a bucket containing 1 HelloToken is returned
            self.sample_vault.take(1)
        }
```

Para ejecutarlo solo debes indicarlo de la siguiente manera, en el terminal: 
```rust
resim call-method $component free_token
```

```
PS C:\Users\mi_usuario\radixdlt-scrypto\miprimerblueprint> resim call-method $component free_token
Transaction Status: SUCCESS
Execution Time: 26 ms
Instructions:
├─ CallMethod { component_address: 02a2a79aa613da237bcda37fd79af36e09eadd195976092cb24696, method: "free_token", args: [] }
└─ CallMethodWithAllResources { component_address: 0293c502780e23621475989d707cd8128e4506362e5fed6ac0c00a, method: "deposit_batch" }
Instruction Outputs:
├─ Bucket(1024u32)
└─ ()
Logs: 1
└─ [INFO ] My balance is: 1000 HelloToken. Now giving away a token!
New Entities: 0
```
¿Qué ha sucedido? 

<div class="alert alert-success" role="warning">
 ¡FELICIDADES!, ya has ejecutado tu primer Contrato Inteligente de Scrypto 🥳🎉🎉🎉🎉
</div>

Después de la alegría veamos en profundidad que sucedió, en realidad lo esperado: que nos devolvió un mensaje con el balance de 'HelloToken' y después tomo 1... ¿Pero quien lo tomo?
fue depositado en la cuenta que creaste al inicio de esta unidad, escribe el comando:
```
resim show $account
``` 
Y nos devolverá:
```
Component: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a
Blueprint: { package_address: 010000000000000000000000000000000000000000000000000003, blueprint_name: "Account" }
Authorization
├─ "deposit_batch" => AllowAll
└─ "deposit" => AllowAll
State: Struct(LazyMap("bc417218214859fbbf019072394c50cc53d5419f4acd7a660dc7c880f0cce31a02040000"))
Lazy Map: 020d3869346218a5e8deaaf2001216dc00fcacb79fb43e30ded79a(bc417218214859fbbf019072394c50cc53d5419f4acd7a660dc7c880f0cce31a, 1026)
├─ ResourceAddress("030000000000000000000000000000000000000000000000000004") => Vault("bc417218214859fbbf019072394c50cc53d5419f4acd7a660dc7c880f0cce31a03040000")
└─ ResourceAddress("0394a0e64bbd357867d9e4904a7011b9977fcbb01c87c80a908896") => Vault("42f0acd2c566ff28c2862de1c69cd5b6f2e38228dad9b789b543a650c4b070ef02040000")
Resources:
├─ { amount: 1000000, resource address: 030000000000000000000000000000000000000000000000000004, name: "Radix", symbol: "XRD" }
└─ { amount: 1, resource address: 0394a0e64bbd357867d9e4904a7011b9977fcbb01c87c80a908896, name: "HelloToken", symbol: "HT" }
```
Si te fijas en los recursos, vemos que en nuestra cuenta ahora hay dos recursos asociados, el primero que ya teníamos al crear la cuenta (XRD) y ahora un el HelloToken.  

Aclarar que por defecto el Simulador utiliza la primera cuenta creada de forma predefinida para llevar a cabo las transacciones (*Nota: en próximas lecciones veremos como utilizar más de una cuenta*) 

Nuevamente enhorabuena... ya has podido crear y ejecutar tu primer contrato inteligente. En próximas lecciones vas a crear tus propios contratos/componentes, ¡Te atreves!

::: warning Importante:
- Soy muy consciente que hay muchas que no has entendido, ¡TRANQUILO!, las entenderás...
:::

### Bibliografía
- [Ejemplo hello-wold](https://github.com/radixdlt/radixdlt-scrypto/tree/main/examples/hello-world)