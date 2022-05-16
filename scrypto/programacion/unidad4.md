# Badges y cobros - Mejoramos blueprint Temperatura

::: warning
- Esta unidad está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Pronto... Video Demostración]()
:::

En la unidad anterior seguimos los pasos que normalmente se dan en el desarrollo de software, si recuerdas, el último paso: *Mantenimiento*  era cuando realizábamos cambios/mejoras ya sea para resolver problemas detectados o para implementar nuevas funciones requeridas por nuestro cliente. 

En este caso vamos a implementar nuevas funciones y mejoras (fase mantenimiento), veamos:

### Análisis del problema
- **Problema**: La empresa 'Aseguradora' que nos encargo el blueprint, [de la unidad anterior](/scrypto/programacion/unidad3.md), 'Temperatura' esta muy contenta con nuestro trabajo y ahora viendo el potencial que tiene este contrato inteligente quiere mejorarlo. 

    - Aseguradora: En vista de lo bien que va el contrato inteligente para proveer el dato temperatura a la red de Radix (Oráculo), queremos mejorarlo y poder introducir más de una temperatura, junto a el lugar donde se toma y no queremos que cualquiera pueda cambiar la temperatura.

    - Programador: ¿Cuantas personas/otros podrán cambiar la temperatura?

    - Aseguradora: En principio solo una persona, lo mismo en un futuro damos acceso a más ¿es posible?

    - Programador: Claro, es posible. 

    - Aseguradora: También nos gustaría monetizar este contrato, cobrando una pequeña comisión por cada lectura.

    - Programador: El precio será fijo o se podrá cambiar. 

    - Aseguradora: El precio será fijo a *0.001* XRD

    - Programador: ¿El lugar es una ciudad? ¿Guardaremos el nombre?

    - Aseguradora: Si, serán ciudades y nosotros tenemos una [codificación numérica](https://es.wikipedia.org/wiki/Código_postal_de_España) de hasta tres cifras para identificar las ciudades.

- **Análisis**: 
¿Te atreves ha hacerlo por ti mismo? Divide en pequeños sub-problemas bien definidos lo que ha solicitado el cliente 'Aseguradora'.

### Diseño de los algoritmos

¿Te atreves a dibujar tus propios algoritmos? No es difícil, solo necesita ✏️+📄.

### Codificación

#### Sub-Problema 1 - Poder introducir más de una temperatura e incluir lugar

Para poder introducir más de una temperatura y asignar un lugar (numérico) vamos a utilizar los **Hash Maps** que incluye Rust. Es una forma muy eficiente de guardar claves con un valor asociado, la sintaxis es: *HashMap K, V* donde almacena una asignación de claves de tipo K a valores de tipo V. 

::: tip 
Los mapas Hash (hash map) son útiles cuando se quiere buscar datos no usando un índice, sino usando una clave que puede ser de cualquier tipo.
:::

En nuestro caso, la clave (k) sera el lugar donde se toma la temperatura y el valor (v) la temperatura. Como estos datos debe guardarse de forma permanente dentro del *Componente* los vamos a declarar dentro de la *Estructura* del blueprint:

```rust
struct Temperatura {
    ciudad_temperatura: HashMap<u16,Decimal>
}
```
Si te fijaste hemos cambiado el nombre de la variable *temp* a *ciudad_temperatura* solo por ser esclarecedor, y no excesivo, en la información que se transmite con el nombre de la variable. 

Ahora ya no solo guardamos un *Decimal*, ahora gracias a *HashMap* guardamos también un entero (sin signo) de tipo u16 (u8 se quedaba corto) como clave para la temperatura, recuerda que el cliente 'Aseguradora' tiene una correspondencia entre esta clave y ciudades. 

Al codificar cambios en la estructura nos vemos obligados a realizarlos también en la *Implementación* (impl) quedando de la siguiente manera:

```rust
impl Temperatura {
    pub fn instantiate_temperatura() -> ComponentAddress {
        Self {
            ciudad_temperatura: HashMap::new()
        }
        .instantiate()
        .globalize()
    }
```
Ya no es necesario introducir una temperatura al crear el *Componente* sino que inicializamos el HasMap con el siguiente código:
 ```rust
 HashMap::new()
 ```
 Al método *cambiar_temp* le cambiaremos el nombre por *nueva_temperatura*, que es mas descriptivo para su uso actual, y además incluiremos el argumento para pasar la ciudad que hace de clave (k) en el HashMap quedando así:

 ```rust
pub fn nueva_temperatura(&mut self, ciudad: u16, temp: Decimal) {
        self.ciudad_temperatura.insert(ciudad, temp);
    }
 ```

Los HashMap utilizan la propiedad **.insert** para incluir o modificar un registro. Es decir si no existe la clave que introducimos la agrega como nueva, si ya existe modifica el valor (V) del registro por el nuevo. Esta característica lo hace perfecto para nuestro problema.

El método *leer_temperatura* también sufre cambios, incluyendo algo de lógica y el argumento para ahora solicitar la temperatura de que ciudad queremos ver:

```rust
pub fn leer_temperatura(&mut self, ciudad: u16) -> Decimal {
    assert!(self.ciudad_temperatura.contains_key(&ciudad), "NO existe la ciudad {}", &ciudad);

    let temp_ciudad: Decimal = self.ciudad_temperatura.get(&ciudad).unwrap().clone();

    info!("La temperatura es: {} grados", temp_ciudad);

    temp_ciudad
}
```
Como parámetro de entrada pasamos un dato tipo *u16* que es como hemos guardado el identificador de la ciudad. Finalmente el método devuelve la temperatura en forma de Decimal **(-> Decimal)**. El macro *assert!* nos permite realizar una pregunta, si el resultado es positivo entonces pasamos a ejecutar las siguientes instrucciones por al contrario si es resultado es negativo el método se romperá y devolverá un mensaje de error formateado por nosotros, en este caso: "NO existe esa ciudad" ya que hemos preguntado:

```rust 
self.ciudad_temperatura.contains_key(&ciudad)
```

Lo que hacemos es leer el *HashMap* de la estructura *self.ciudad_temperatura* y preguntamos si contiene la llave *(contains_key(&ciudad))* y le pasamos una referencia (&) a la variable *ciudad*.

::: tip
- &: referencia o puntero a una dirección de memoria que apunta a un valor.
:::

Utilizando el macro *info!* formateamos un mensaje que lanzara un mensaje por consola con el dato de temperatura solicitado a modo informativo.

Devolvemos el dato de tipo Decimal *temp_ciudad*. Recuerda que no ponemos punto y coma final ya que es una expresión que retorna un valor.  


#### Sub-Problema 2 - Integrar seguridad en la introducción de temperaturas

Para conseguir esto Scrypto ha implementado una novedoso sistema de autorización a nivel de sistema, llamado [**Badge/Auth-Zone**](/scrypto/conceptos/conceptos4.md), donde se crean pruebas (Proof) de autorización (Badge) para la realización de acciones.  En nuestro caso el cliente ha pedido que una sola persona tenga acceso al método cambiar_temperatura, primero creemos el *Badge* para ello modificaremos la estructura:

::: danger Proof
- En estos momentos, en la versión 0.4 o mayor, Scrypto permite al menos dos manera de poder pasar la prueba de que poseemos cierta acreditación para la realización de ciertas acciones. Se puede a traves de **Auth-Zone** y los Manifiestos de transacciones y de forma intencional.
:::

En este caso vamos a acreditar la autoridad de forma intencional pidiendo que el cliente pase como argumento su Proof.

```rust
struct Temperatura {
    ciudad_temperatura: HashMap<u16,Decimal>,
    admin_badge: ResourceAddress
}
```

Como veis hemos incluido la variable: *admin_badge*, como tipo **ResourceAddress**, la cual guardará la dirección que identifica el recurso que crea estas insignias con el fin de luego saber quien tiene acceso, y quien no, cuando implementemos la seguridad. ¿Como creamos estas insignias? Parecido a un token, en realidad son un recurso fungible, para ello vamos a cambiar la función constructora *instantiate_temperatura*:

::: tip
Las **banderas** en la creación de recursos en Radix, van a permitir una infinidad de tipos diferentes de recursos inimaginable, junto con la transparencia para que los usuarios sepan en todo momento que tipo de recursos están utilizando. Esto permite entre otras cosas saber por ejemplo si un token pude acuñar más unidades o no con un simple vistazo. En el caso que sigue utilizamos la bandera: *DIVISIBILITY_NONE*, ¡desde luego muy comprensible!, este recurso no permite mandar partes del mismo. 
:::

```rust
pub fn instantiate_temperatura() -> (ComponentAddress, Bucket) {
    let badges: Bucket = ResourceBuilder::new_fungible()
    .divisibility(DIVISIBILITY_NONE)
    .metadata("name", "Admin Badge")
    .initial_supply(1);

    let component = Self {
        ciudad_temperatura: HashMap::new(),
        admin_badge: badges.resource_address()
    }
    .instantiate();
    .globalize();

    (component, badges)
}
```
Como ves hemos realizado diversos cambios a la función *instantiate_temperatura*: primero que todo hemos añadido un parámetro de salida junto que ya teníamos *component* y es un *Bucket* o contenedor temporal de recursos, que es donde le pasaremos la insignia o badge que luego crearemos.

Después hemos declarado una variable, *badges* que hemos inicializado como un constructor de recurso nuevo:

```rust
let badges: Bucket = ResourceBuilder::new_fungible()
```

Y seguidamente le hemos pasado una serie de parámetros: un nombre, su fragcionado y la cantidad de recursos a crear con *initial_supply*. En nuestro caso vamos a crear solo 1.

```rust
.metadata("name", "Admin Badge")
.divisibility(DIVISIBILITY_NONE)
.initial_supply(1);
```
Ya solo nos quedaría implementar la seguridad de ejecución al método *cambiar_temperatura* y es tan fácil como agregar un nuevo parámetro de entrada de tipo **Proof** a la que llamaremos *auth*, junto con un par sentencias de control que garantizaran que la prueba que se pase de forma intencional corresponde con la esperada, recuerda para ello hemos guardado en la estructura del componente la dirección del recurso que crea las insignias (badges).

```rust
pub fn nueva_temperatura(&mut self, ciudad: u16, temp: Decimal, auth: Proof) {
    assert_eq!(auth.resource_address(), self.admin_badge, "Insignia incorrecta.");
    assert!(auth.amount() >= dec!("1"), "Debes pasar al menos una insignia.");
    self.ciudad_temperatura.insert(ciudad, temp);
}
```

Las macros tipo **assert** de Rust nos permiten realizar una serie de comprobaciones que si no se cumplen, se lanzara una excepción, deteniendo el proceso. En este caso, vamos a comprobar que la insignia que se pasa como argumento de la función *nueva_temperatura* sea la misma que la que se creo en la función *instantiate_temperatura* y paso a nuestra cuenta. Para ello utilizamos la macro **assert_eq!** que nos permite comparar dos valores, en este caso la *dirección* (ResourceAddress) de la insignia que se pasa como argumento tipo Proof a la función *nueva_temperatura* y la dirección que guardamos en admin_badge, recuerda: a la que accedemos gracias a *self*.

```rust	
assert_eq!(auth.resource_address(), self.admin_badge, "Insignia incorrecta.");
```
Si no hay equivalencia entre ambas direcciones, entonces la prueba no es correcta y se lanza una excepción con el siguiente mensaje de error: "Insignia incorrecta."

A continuación vamos a comprobar que la cantidad de insignias que se pasa como argumento de la función *nueva_temperatura* sea mayor o igual a 1, para ello utilizamos la macro **assert!** que nos permite realizar una serie de comprobaciones, en este caso, que la cantidad de insignias sea mayor o igual a 1.

```rust	
assert!(auth.amount() >= dec!("1"), "Debes pasar al menos una insignia.");
```
Con el método **amount()** podemos obtener la cantidad total contenida en un recurso tipo: Vault, Bucket y Proof.

::: tip dec!
La macro **dec!** nos permite crear un decimal a partir de un literal.
```rust	
let a = dec!(1);
let b = dec!("1.1");
```
:::

Ahora solo aquella persona que pueda probar (proof) que tiene el badge requerido podrá cambiar la temperatura de las ciudades.

*Ojo: Existe otra manera de hacer esto mismo que ya analizaremos en próximas unidades cuando veamos como crear nuestras propios manifiestos de transacciones.*

#### Sub-Problema 3 - Cobrar comisión por leer temperatura

El último requerimiento, para esta versión del *blueprint*, es cobrar *0.001XRD* por leer la temperatura de la ciudad o lugar solicitados:

Para ello primero es necesario crear un contenedor permanente donde recoger y guardar los fees cobrados por leer la temperatura. Esto lo creamos en la estructura, y ya que estamos creamos una variable que guarde el dato del precio que cobraremos:

```rust
struct Temperatura {
    ciudad_temperatura: HashMap<i16,Decimal>,
    cambiar_temp_badge: ResourceDef,
    pool_xrd: Vault,
    precio_ver: Decimal
}
```
Al contenedor de XRD lo hemos llamado *pool_xrd* y declarando del tipo *Vault* o contenedor permanente. La variable que guardará el dato del precio la llamaremos *precio_ver* y es declarada de tipo *Decimal* ya que el precio puede contener decimales. 

A continuación debemos realizar cambios en la función constructora *instantiate_temperatura*, primero para guardar el dato del *precio* al instanciar el componente y segundo para crear el contenedor de XRD que guardara las fees cobradas por el uso del Oráculo. Lo haremos así:

```rust
pub fn instantiate_temperatura(precio: Decimal) -> (Component, Bucket) {

    let badges: Bucket = ResourceBuilder::new_fungible()
        .divisibility(DIVISIBILITY_NONE)
        .metadata("name", "Admin Badge")
        .initial_supply(1);

    let component = Self {
        ciudad_temperatura: HashMap::new(),
        admin_badge: badges.resource_address(),
        pool_xrd: Vault::new(RADIX_TOKEN),
        precio_ver: precio
    }
    .instantiate();
    .globalize();

    (component, insignia_cambiar_temp)
}
```
Si te fijaste, ahora la función *instantiate_temperatura* acepta un parámetro de entrada, dentro de los paréntesis(), de tipo Decimal al que llamamos *precio*. Luego simplemente como ya hemos hecho anteriormente pasamos el valor que nos han dado a traves de la la variable *precio* a la variable que hemos declarado en la estructura de forma permanente *precio_ver*:

```rust
precio_ver: Decimal
```

Finalmente vamos a cambiar el método *leer_temperatura* para que acepte pagos, el código quedaría de la siguiente manera:

```rust
pub fn leer_temp(&mut self, ciudad: i16, mut pago: Bucket) -> (Bucket, Decimal) {
    assert!(self.ciudad_temperatura.contains_key(&ciudad), "NO existe esa ciudad");
    assert!(pago.amount() >= self.precio_ver, "Saldo insuficiente");

    self.pool_xrd.put(pago.take(self.precio_ver));

    let temp_ciudad: Decimal = self.ciudad_temperatura.get(&ciudad).unwrap().clone();

    info!("La temperatura es: {} grados", temp_ciudad);

    (pago, temp_ciudad)
}
```
En principio el primer cambio lo encontramos en los parámetros de entrada de la función, donde hemos incluido el parámetro *pago* de tipo *Bucket*, ya sabes que Bucket es un contenedor temporal donde los usuarios nos pasaran los XRD para pagar el acceso al dato temperatura. Seguidamente hemos incluido en la sentencia/macro *assert!* la condición: *pago.amount() >= self.precio_ver* donde la propiedad *.amount()* devuelve el saldo total del *Bucket* y esta a su vez se compara con el operador *mayor o igual que (>=)* el precio indicado al instanciar el componente.

Esta nueva sentencia permite saber si el usuario ha incluido en la transacción suficiente saldo como para pagar la comisión por leer el dato temperatura. 

Seguidamente añadimos la sentencia:

```rust
self.pool_xrd.put(pago.take(self.precio_ver));
```
Donde lo que hacemos es guardar dentro del contendedor para xrd, que creamos en la estructura del  *componente* y al que llamamos *pool_xrd*, el precio que se ha asignado a leer el dato temperatura. Si te das cuenta solo tomamos el valor requerido y no mas de lo enviado en la transacción por el usuario. Por eso al final del método *leer_temp* encontramos la expresión (recuerda una expresión devuelve valor y por eso no se pone punto y coma al final):

```rust
(pago, temp_ciudad)
```
*pago* que es un *Bucket*. Y por eso este método devuelve un *Bucket* como vemos al inicio de la función después del guion flecha **->**:

```rust
pub fn leer_temp(&self, ciudad: i16, pago: Bucket) -> (Bucket, Decimal)
```

Finalmente el código debería haberte quedado así:

```rust
use scrypto::prelude::*;

blueprint! {
    struct Temperatura {
        ciudad_temperatura: HashMap<u16,Decimal>,
        admin_badge: ResourceAddress,
        pool_xrd: Vault,
        precio_ver: Decimal
    }
    impl Temperatura {
        pub fn instantiate_temperatura(precio: Decimal) -> (ComponentAddress, Bucket) {

            let badges: Bucket = ResourceBuilder::new_fungible()
            .divisibility(DIVISIBILITY_NONE)
            .metadata("name", "Admin Badge")
            .initial_supply(1);
            

            let componente = Self {
                ciudad_temperatura: HashMap::new(),
                admin_badge: badges.resource_address(),
                pool_xrd: Vault::new(RADIX_TOKEN),
                precio_ver: precio
            }
            .instantiate()
            .globalize();

            (componente, badges)
        }

        pub fn leer_temperatura(&mut self, ciudad: u16, mut pago: Bucket) -> (Bucket, Decimal) {
            assert!(self.ciudad_temperatura.contains_key(&ciudad), "NO existe la ciudad {}", &ciudad);
            assert!(pago.amount() >= self.precio_ver, "Saldo insuficiente");

            self.pool_xrd.put(pago.take(self.precio_ver));

            let temp_ciudad: Decimal = self.ciudad_temperatura.get(&ciudad).unwrap().clone();

            info!("La temperatura es: {} grados", temp_ciudad);
        
            (pago, temp_ciudad)
        }

        pub fn nueva_temperatura(&mut self, ciudad: u16, temp: Decimal, auth: Proof) {
            assert_eq!(auth.resource_address(), self.admin_badge, "Insignia incorrecta.");
            assert!(auth.amount() >= dec!("1"), "Debes pasar al menos una insignia.");
            
            self.ciudad_temperatura.insert(ciudad, temp);
        }
    }
}
```

Con esto habríamos concluido la codificación de todas las especificaciones requeridas por el cliente 'Aseguradora'.


### Compilación y ejecución

En esta sección os voy a dejar solo los comandos y sub-comandos necesarios 🤪:

- Para limpiar los datos en memoria del *Simulador*:
```
resim reset
```
- Para crear una nueva cuenta:
```
resim new-account
```
- Recuerda guardar las direcciones en variables del sistema, aquí te dejamos el listado de variables que tendrás que usar:
```rust
set <nombre variable> = <direccion>
/* variables:
    - package -> set package xxxxxxxxxxxxxxxxxxxxxxxx
    - component -> set component xxxxxxxxxxxxxxxxxxxxxxx
    - account -> set account xxxxxxxxxxxxxxxxxxxxxxx
    - xrd -> set xrd xxxxxxxxxxxxxxxxxxxxxxxx
    (el ResourceAddress de xrd lo encontramos en la cuenta)
    - badge(ResourceAddress) -> set badge xxxxxxxxxxxxxxxxxxx
    (el ResourceAddress del badge lo encontramos en la cuenta)
*/
```
- Ver datos cuenta y guardar por ejemplo la referencia a los xrd que tenemos:
```
resim show $account
```
- Para publicar un Package
```
resim publish .
```
::: tip 
- Para modificar un Package ya publicado sin necesidad de cambiar la direccion (*Donde $package es el nombre arbitrario que hemos utilizado hasta ahora en el curso*)
```
resim publish . --address $package 
```
::: 

- Instanciar un componente pasando como argumento el precio de lectura:
```
resim call-function $package Temperatura instantiate_temperatura 0.001
```
- Llamar al método nueva_temperatura pasando argumentos: código ciudad, temperatura e insignia
```rust 
resim call-method $component nueva_temperatura 10 20.5 1,$badge
// Recuerda que para insertar valores negativos ponemos delante dos guiones --
// resim call-method $component nueva_temperatura 10 -- -5.2 1,$badge
```
- Llamar al método leer_temperatura pasando argumentos:  codigo ciudad y tokens para el pago en XRD
```
resim call-method $component leer_temperatura 10 0.001,$xrd
```

Seguro que con estos comandos lo tenéis muy fácil 😅  


¿Ya lo has conseguido?
Enhorabuena... ya has podido mejorar un blueprint!! implementando nuevas funciones. Además en esta unidad ya te he dejado un poco mas de libertad para que vueles solo. En el vídeo yo ire paso a paso para que lo veas todo muy muy claro. Si has llegado hasta aquí no puedes parar, lo más difícil ya esta superado. En las siguientes unidades seguiremos escribiendo *blueprint*, para que vayas cogiendo soltura con mas comandos y funciones de Scrypto. 

::: warning Importante:
- Soy muy consciente de que hay muchas que no has entendido, ¡TRANQUILO!, no te rindas, las entenderás... 😉
:::