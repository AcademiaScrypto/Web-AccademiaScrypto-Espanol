# Lista de Boda - Registros

::: warning
- Esta unidad **NO** está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Video Demostración](https://youtu.be/0cfxs3pVmcY)
:::

Un ejemplo de como transformar uno de esos proyectos web1 (estático) y web2 (dinámico) en web3 (descentralizado). En este caso el típico *libro de visitas* o Guestbook que era muy usual en las primeras webs. Vamos a darle un vuelta de rosca y vamos a crear un componente que permita construir una web para listas de bodas. 

### Análisis
**Problema/Análisis:** Un 'Wedding Planner' nos ha pedido que creemos una dApp que de servicio a las personas que se van a casar y quieran tener un lugar donde sus familiares y amistades puedan realizar un regalo dejando su nombre y un pequeño comentario. El regalo será en XRD 🤑. Y un mecanismo donde los novios puedan guardar los XRD y poder sacarlos cuando ellos quieran.

- Crear repositorio donde se pueda guardar un nombre, comentario y regalo (XRD)
- Poder leer el listado de registros y un total de XRD regalados
- Poder sacar fácilmente los XRD a una cuenta de los novios.

### Diseño
![Diagramas](./img/diagrama_8.png)

- Una función constructora donde vamos a crear un contenedor para guardar los xrd que nos regalen y una un lugar donde guardar cuatro campos: identificador, nombre, comentario. Al crear el componente se guardará una dirección donde se enviaran los xrd.
- Método para guardar 
- Un método que pintará por pantalla un listado con los registros y un sumatorio del total regalado.
- Un método para que puedan sacar los xrd a su billetera de forma segura.

### Programación
Vamos a llamar este *package* **LibroBoda**, recuerda que los nombres son algo arbitrario. Eso no quiere decir que debamos ser lo mas concisos pero expresivos con el fin de hacer el código legible y fácil de mantener. 

::: tip
- Rust recomienda que si el nombre de una variable lleva más de dos palabras, se debe usar guión bajo (_) para separarlas. 
:::

### La estructura:

```rust
struct ListaBoda {
    registro: HashMap<u128, (String,String,Decimal)>,
    regalos: Vault,
    admin_def: ResourceDef
}
```

La colección [**HashMap**](https://radixdlt.github.io/radixdlt-scrypto/scrypto/rust/collections/struct.HashMap.html) ya la utilizamos en la [unidad 5](http://localhost:8080/scrypto/programacion/unidad4.html) y vimos que era una colección tipo *clave-valor*. En este caso hemos declarado una clave uuid para que sea única y un valor multiple que lo conseguimos gracias a utilizar otro tipo de colección *Tupple* que se usa para almacenar diferentes tipos de datos.

Ademas guardamos la definición del recurso fungible que vamos a crear como insignia para que los novios puedan rescatar los regalos de sus invitados. 

### Función constructora *new*
```rust
pub fn new() -> (Component, Bucket) {
    let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
        .metadata("name", "Admin Lista de Boda")
        .initial_supply_fungible(2);

    Self {
        registro: HashMap::new(),
        regalos: Vault::new(RADIX_TOKEN), 
         admin_def: admin.resource_def()
    }
    .instantiate()

    (comp, admin)
}
```
Para empezar vamos a crear un recurso fungible que nos servirá como insignia que permitirá a los novios poder sacar los xrd regalados.

```rust
let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
    .metadata("name", "Admin Lista de Boda")
    .initial_supply_fungible(2);
```
Creamos una variable de tipo *bucket* que contendrá los recursos fungibles. El recurso fungible lo declaramos como no divisible, le asignamos un nombre con un metadata y finalmente definimos el supply en 2 unidades.

Seguidamente inicializamos las variables de la estructura como ya hemos explicado en unidades anteriores.

Recordad que *Vault* es un contenedor especial de Scrypto y Radix Engine donde se deben guardar, siempre, los recursos. En este caso *Vault* es un contenedor permanente. Para poder inicializarlo en la función constructora utilizamos el *método* [**new**](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Vault.html#method.new) de la siguiente manera:

```rust
Vault::new(RADIX_TOKEN)
```

::: tip
- RADIX_TOKEN: nos simplifica el trabajo de definición de la referencia de recurso XRD
:::

### Método *nuevo_registro*
```rust
pub fn nuevo_registro(&mut self, nombre: String, comentario: String, regalo: Bucket) -> Bucket {
    self.registro.insert(Uuid::generate(),(nombre,comentario, regalo.amount()));
    self.regalos.put(regalo.take(regalo.amount()));
    regalo
}
```

Este método requiere que pasemos los datos necesarios para agregar un nuevo registro: nombre, comentario y xrd. La llamada a este método deberia ser algo así:
```
resim call-method $package nuevo_registro Andres "Comentario a los novios" 0,$xrd
```
En este ejemplo no le regalamos ningún xrd a los novios y si te das cuenta cuando queremos pasar a una variable tipo *String* mas de una palabra las entrecomillamos. 

Ahora estos datos se insertan en el HashMap *registro* que hemos creado en la estructura. Primero generamos un uuid (identificador único) y luego entre parentesis (Tupla) el nombre, comentarios y la cantidad de xrd que regalamos. Como ves para agregar un nuevo registro es tan sencillo como utilizar el método [**insert()**](https://radixdlt.github.io/radixdlt-scrypto/scrypto/rust/collections/struct.HashMap.html#method.insert):

```rust
self.registro.insert(Uuid::generate(),(nombre,comentario, regalo.amount()));
```

::: tip
- Para conocer la cantidad de tokens que tienes en un Vault o Bucket debes utilizar **amount()**
:::

Una vez tenemos el registro guardado tomamos los xrd y los pasamos al contenedor *regalos*, tan sencillo como *take* y *put*

```rust
self.regalos.put(regalo.take(regalo.amount()));
```

::: tip
- El método [**take_all()**](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Vault.html#method.take_all) solo se puede utilizar con los contenedores permanentes tipo *Vault*
:::

Finalmente la última línea devuelve el Backet *regalo* aunque lo hayamos vaciado. La idea siempre es devolver/quemar los contenedores temporales aunque estén vacíos. Esto permite asegurar la transición correcta de los activos de un lugar a otro dentro de la red y gracias a la [FSM de la que ya hablamos en los fundamentos de Radix](/radix/unidad2.md). 

```rust
regalo
```
::: warning Recuerda
- Solo cerramos con punto y coma (**;**) las sentencias. Las expresiones retornan un valor.
:::

### Método *leer_registros*

*(Nota: Esta no es la mejor practica. En realidad los contratos/componentes no están pensados para sacar listados por pantalla, lo suyo seria devolver una colección con los registros para ser tratados/pintados desde el frontend. En este caso lo hacemos para que tengáis una percepción mas real de lo que sucede, y solo a modo de ejemplo inicial.)*

```rust
pub fn leer_registros(&self) {
    let mut total_regalos = Decimal::zero();
    info!("Comentarios:");
    info!("--------------------------------------------------");
    for (_uuid, comentario) in &self.registro {
        info!("{:?}", comentario);
        total_regalos += comentario.2;
    };
    info!("--------------------------------------------------");
    info!("Total: {} xrd regalados" , total_regalos);
}
```

Creamos una nueva variable mutable que iniciamos como tipo Decimal y con valor 0:

```rust
let mut total_regalos = Decimal::zero();
```
[**Decimal::zero()**](https://radixdlt.github.io/radixdlt-scrypto/scrypto/types/struct.Decimal.html#method.zero): Devuelve un valor 0 de tipo decimal. (Nota: Que no es lo mismo que poner = 0 directamente, te invito a que lo pruebes y compruebes por ti mismo el conflicto entre tipo de datos)

[info!](https://radixdlt.github.io/radixdlt-scrypto/scrypto/core/struct.Logger.html#method.info): este método emite un mensaje de tipo informativo. Scrypto tiene una utilidad en su core [Logger](https://radixdlt.github.io/radixdlt-scrypto/scrypto/core/struct.Logger.html#) que permite emitir diferentes tipos de mensajes: log, trace, debug, into, warn, error.

```rust
for (_uuid, comentario) in &self.registro {
    info!("{:?}", comentario);
    total_regalos += comentario.2;
};
```
En este [bucle](/rust/bucles.md) iteramos sobre la colección *registro*, pintando cada comentario y sumando el valor del regalo para finalmente pintar por pantalla el total de xrd que nos han regalado. *Ojo: el guión bajo inicial en **_uuid** lo utilizamos en variables que no vamos a utilizar más adelante.*

::: tip
- Para acceder a un elemento concreto dentro de la estructura de una *Tupla* utilizamos el indice precedido de un punto, donde 0 es el primer elemento:
```rust
let ejemplo_tupla = ("hola", 2);
println!("ejemplo: {}", ejemplo_tupla.1);
// ejemplo: 2
```
:::

### Método *sacar_todo*
```rust
#[auth(admin_def)]
pub fn sacar_todo(&mut self) -> Bucket {
    self.regalos.take_all()
}
```
Finalmente cuando los novios quieran sacar todos sus regalos a su billetera personal lo podrán hacer usando este método. Como veis hemos aplicado los patrones de diseño que nos han recomendado, por un lado utilizar insignias para acceder a ciertos métodos y luego no devolver los recursos inmediatamente sino dejarlos en el componente hasta que los usuarios, en este caso novios, quieran sacarlos. 

Recuerda que inicialmente al instanciar el *componente* creamos una recurso fungible con un supply de 2 unidades y guardamos su *definicion* para después poder utilizalo como insignia de acceso a ciertos métodos.

Aquellos que posean esa insignia podrán retirar los fondos. *(En la próxima unidad veremos algún método de consenso para la retirada 👫)*

```rust
self.regalos.take_all()
```
Como hemos visto anteriormente el recurso *Vault* admite el método [**take_all()**](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Vault.html#method.take_all) para tomar todos los recursos y en este caso devolverlos.

### Código completo:
```rust
use scrypto::prelude::*;

blueprint! {
    struct ListaBoda {
        registro: HashMap<u128, (String,String,Decimal)>,
        regalos: Vault,
        admin_def: ResourceDef
    }

    impl ListaBoda {
        pub fn new() -> (Component, Bucket) {
            let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
            .metadata("name", "Admin Lista de Boda")
            .initial_supply_fungible(2);
            
            let comp = Self {
                registro: HashMap::new(),
                regalos: Vault::new(RADIX_TOKEN),
                admin_def: admin.resource_def()
            }
            .instantiate();

            (comp, admin)
        }

        pub fn nuevo(&mut self, nombre: String, comentario: String, mut regalo: Bucket) -> Bucket {
            self.registro.insert(Uuid::generate(),(nombre,comentario, regalo.amount()));
            self.regalos.put(regalo.take(regalo.amount()));
            regalo
        }

        pub fn leer_registros(&self) {
            let mut total_regalos = Decimal::zero();
            info!("Comentarios:");
            info!("--------------------------------------------------");
            for (_uuid, comentario) in &self.registro {
                info!("{:?}", comentario);
                total_regalos += comentario.2;
            };
            info!("--------------------------------------------------");
            info!("Total: {} xrd regalados" , total_regalos);
        }

        #[auth(admin_def)]
        pub fn sacar_todo(&mut self) -> Bucket {
           self.regalos.take_all()
        }
    }
}
```

### Compilación y ejecución
A estas alturas seguro que ya sabes publicar el package, instanciar el component y llamar a las funciones pasando un parámetro.

::: details Pero... por si eres un Homo Emilius te dejo una ayudita 🙈.
1. Limpiar el simulador
```rust
resim reset
```
2. Crear un Package
```
scrypto new-package LibroBoda
cd LibroBoda
```
3. Crear una cuenta (recuerda copiar la dirección de los XRD de tu cuenta)
```rust
resim new-account
set acct [Address de la cuenta que acabamos de crear]
set pub [Guardamos la clave pública de esta cuenta]
set xrd 030000000000000000000000000000000000000000000000000004
```
4. Copiar o escribir el código (recuerda guardar ctrl + s)
- Recuerda guardar el código de este ejercicio dentro del archivo lib.rs que has creado en la carpeta *\radixdlt-scrypto\LibroBoda\src\lib.rs*
5. Publicar y guardamos la dirección del Package
```rust
resim publish .
set pack [New Package Reference]
```
6. Instanciar componente (recuerda que en este caso hay que añadir el argumento del precio) y guardar la dirección del componente.
```rust
resim call-function $pack ListaBoda new
set comp [dirección del componente]
```
7. Probar método *nuevo*
```rust
resim call-method $comp nuevo "Emilio Bitcoin" "Que boda mas bonita, os deseo muchos hijos..." 1000,$xrd
// Ojo que cuando pasamos un dato de tipo string y tiene mas de una palabra debemos entrecomillarlo.
```
8. Probar método *leer_registros*
```rust
resim call-method $comp leer_registros
```
9. Sacar los regalos del componente a la billetera que lo ejecuta
```rust
resim call-method $comp sacar_todo 1,[definición del recurso fungible Admin Lista de Boda]
```
10. Comprobamos que hemos recibido los xrd
```rust
resim show $acct
```
:::

😎 ¿Qué tal?, ¿Te gusto este ejemplo?... la semana que viene lo completaremos añadiendo algo de seguridad y agradecimiento de por parte de los novios. Con la próxima unidad, la décima, terminaremos la estructura básica de aprendizaje. Si has llegado hasta aquí un poco mas de esfuerzo y podrás decir que sabes lo fundamental como para comenzar a escribir tus componentes. 
