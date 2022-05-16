# Lista de Boda II + NFT 

::: warning
- Esta unidad **NO** está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Video Demostración](https://youtu.be/PCO0faacShw)
:::

Seguimos con el ejemplo de *Guestbook* enmarcado en una *lista de bodas* donde hasta el momento hemos podido mandar un mensaje a los novios y un regalo económico (XRD). El ejemplo de hoy no es más que una vuelta de rosca a código que ya hemos escrito anteriormente 😎.

::: warning Opinión
- Al final el código es repetitivo ya que las soluciones suelen tener características comunes. Lo mismo que una casa siempre tiene puertas, ventanas, paredes, luces... las aplicaciones siempre tienen elementos comunes que debemos conocer e incorporar de la forma mas útil, esto lo podemos lograr respetando y utilizando patrones de diseño.
:::

### Análisis
**Problema/Análisis:** 

Muchos novios dan detalles/regalos a sus invitados para que recuerden ese día. Nos piden incorporar a nuestro *componente* la capacidad de generar un recuerdo NFT a todos aquellos que registren un mensaje. Y como en muchas ocasiones, pasado un tiempo, las bodas terminan en separaciones, queremos poder registrar ese dato de fin del enlace y que los NFT que se dieron como regalo pudieran actualizar la información. Además nos piden mejorar algo el sistema de retirada de fondos para que de alguna manera la pareja llegue a un consenso para dicha retirada.

- Poder generar NFT con datos mutables e inmutables.
- Permitir guardar fecha de fin del enlace.

### Diseño
*Nota: En muchas ocasiones podemos realizar el diseño con psudocódigo que es una representación escrita de un algoritmo, es decir, muestra en forma de texto los pasos a seguir para solucionar un problema.*

- Función constructora
    - Crear insignia para administradores (novios)
    - Creer definición NFT
    - Autorización acuñar NFT
    - Crear Tabla de registros
    - Crear deposito XRD (regalos)
    - Fecha boda inicio y fin

- Agregar registros
    - Insertar a la tabla registros: nombre, comentario y regalo 
    - Guardar regalo en deposito
    - Acuñar NFT personalizado con el nombre del registro
    - Pasar a la cuenta del invitado

- Leer registros
    - Sacar por pantalla listado de registros

- Sacar XRD de componente
    - Buscar consenso para mandar el monto total de XRD (restringido al admin)

- Actualizar fecha fin boda
    - Método para introducir fecha fin de boda (restringido al admin)

- Actualizar NFT
    - Método para cambiar datos mutables en NFT (fecha boda fin)

### Programación
Como en ocasiones anteriores solo nos centraremos en los cambios realizados en el código con respecto a la [unidad anterior](/scrypto/programacion/unidad8.md) y las nuevas mejoras.

### La estructura:

```rust
#[derive(NonFungibleData)]
struct DatosNft {
    mensaje_detalle: String,
    fecha_boda: String,
    #[scrypto(mutable)]
    fecha_fin: String
}
```
Cuando creamos NFT podemos crear una estructura donde se guardaran los metadatos mutables e inmutables. Para ello Scrypto nos proporciona un *macro* específico: **#[derive(NonFungibleData)]**.

::: tip
- Por defecto todos los datos serán inmutables.
- Si queremos que los datos sean mutables debemos escribir delante (en la línea anterior) del dato **#[scrypto(mutable)]**
:::

En este caso el NFT que daran los novios como detalle por asistir a su boda y dejar un mensaje en este libro de visitas, contendrá un mensaje de los novios, la fecha del enlace, y la fecha de fin del matrimonio. Si te fijaste, este último dato, el de la fecha de fin, es mutable 🙄.

```rust
blueprint! {
    struct ListaBoda {
        registro: HashMap<u128, (String,String,Decimal)>,
        regalos: Vault,
        boda_nft_def: ResourceDef,
        auth_nft: Vault,
        fecha_boda: String,
        fecha_fin: String,
        admin_def: ResourceDef,
        aprobacion: u8
    }
```
Con respecto a la unidad anterior, hemos agregado:
    1. Un contendedor (Vault) para guardar la insignia que tiene la autorización para mintear nft en este componente.
    2. Declaramos dos variables para guardar las fechas de inicio y fin de boda, son de tipo String para ser mas sencillo su implementación desde el simulador. Cuando tengamos un front-end podremos cambiar este tipo de dato a uno mas concreto para guardar fechas. 
    3. Guardaremos en el componente la *Definición" de la insignia de administrador con el fin de implementar un sistema de restricción de accesos a ciertos métodos.
    4. Finalmente para poder crear un sistema, muy sencillito y primitivo, de consenso para sacar los XRD regalados guardamos la cantidad mínima de insignias que hemos creado como administradores para llegar a un consenso. 


### Función constructora *new*

```rust
pub fn new(descripcion_nft: String, fecha_boda: String) -> (Bucket, Component) {

    let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
        .metadata("name", "Admin Lista de Boda")
        .initial_supply_fungible(2);

    let auth_nft = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Autorizacion para crear nft boda")
                .initial_supply_fungible(1);

    let boda_nft_def = ResourceBuilder::new_non_fungible()
                        .metadata("name", descripcion_nft)
                        .flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE)
                        .badge(auth_nft.resource_def(), MAY_MINT | MAY_CHANGE_INDIVIDUAL_METADATA)
                        .no_initial_supply();
    
    let comp = Self {
        registro: HashMap::new(),
        regalos: Vault::new(RADIX_TOKEN),
        boda_nft_def,
        auth_nft: Vault::with_bucket(auth_nft),
        fecha_boda: String::from(fecha_boda),
        fecha_fin: String::new(),
        admin_def: admin.resource_def(),
        aprobacion: 2
    }
    .instantiate();

    (admin, comp)
}
```

En realidad en esta función constructora no hay nada que antes no hayamos codificado:
    1. Creamos un recurso fungible como control de acceso a ciertos métodos que solo podrán accionar los administradores de este componente.
    2. Creamos otro recurso fungible que, en esta ocasión, actuará como seguridad para la acuñación y cambio de los metadatos de los NFT.
    3. Definiremos los NFT y guardarmos dicha definición para despues poder acuñar y modificar los non fungible 'hijos' de esta definición.
    4. Finalmente inicializamos las variables de la estructura. 


### Método *nuevo*
```rust
pub fn nuevo(&mut self, nombre: String, comentario: String, mut regalo: Bucket) -> (Bucket, Bucket) {
    let mensaje = "Gracias por asistir a nuestra boda ".to_string() + &nombre;

    self.registro.insert(Uuid::generate(),(nombre,comentario, regalo.amount()));

    self.regalos.put(regalo.take(regalo.amount()));

    let nft_boda: Bucket = self.auth_nft.authorize(|badge| {
        self.boda_nft_def.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), DatosNft{
            mensaje_detalle: mensaje,
            fecha_boda: self.fecha_boda.to_string(),
            fecha_fin: String::new()
        }, badge)
    });

    (regalo, nft_boda)
}
```

Este método permite a los invitados crear un registro de visitas junto con un comentario y un regalo (XRD). Tales datos y recursos serán enviados con la petición, la cual recibirá de vuelta un NFT si todo va bien!!!

```rust
let mensaje = "Gracias por asistir a nuestra boda ".to_string() + &nombre;
```
Guardamos en una variable *mensajes* un mensaje prefijado junto con el nombre que la persona ha pasado como dato.

```rust
self.registro.insert(Uuid::generate(),(nombre,comentario, regalo.amount()));
```
Guardamos los datos que nos han pasado en la petición como valor y como clave generamos un identificador único Uuid.
::: tip
- Para agregar nuevos registros en una colección como HashMap utilizamos el método **insert**.
:::
```rust
 self.regalos.put(regalo.take(regalo.amount()));
```
Tomamos (take) la cantidad completa del regalo en forma de XRD que fue enviado junto la petición y lo guardamos (put) dentro del contenedor permanente *regalos*. Recuerda que con el método amount() podemos conocer la cantidad de un recurso dentro de un contenedor.

```rust
let nft_boda: Bucket = self.auth_nft.authorize(|badge| {
    self.boda_nft_def.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), DatosNft{
        mensaje_detalle: mensaje,
        fecha_boda: self.fecha_boda.to_string(),
        fecha_fin: String::new()
    }, badge)
});
```
Creamos el NFT a devolver gracias a que contamos con la autorización guardad en el propio componente dentro del contenedor *auth_nft*. Tomamos la autorización y la denominamos *badge* y ya podemos acuñar (mint_non_fungible) el NFT con la definición que creamos (boda_nft_def) con la función *new*. Para acuñar es necesario primero generar/pasar un identificador que envolveremos dentro del tipo *NonFungibleKey*, que se ha creado Scrypto para facilitarnos este proceso. Segundo, pasaremos los datos mutables e inmutables al NFT que hemos diseñado en la estructura *DatosNft*. Y finalmente, tercero, utilizaremos la autorización (*badge*) necesaria para poder generar el NFT.

```rust
(regalo, nft_boda)
```
Recuerda que no podemos dejar dentro del método ningún contenedor, los hemos de devolver todos o eliminar. En este caso devolvemos el contenedor *regalo* aunque este vacio y el contenedor que contiene el NFT generado *nft_boda*.

### Método *fin_boda*
```rust
#[auth(admin_def)]
pub fn fin_boda(&mut self, fin: String) {
    self.fecha_fin = fin.to_string();
}
```
Este método viene precedido del macro *auth* que permite otorgar seguridad. En este caso estamos indicando que solo aquellos que posean una insignia con la definición *admin_def* podrán ejecutar este método.

```rust
self.fecha_fin = fin.to_string();
```
Este método es sencillo, pasamos un String y actualizamos el dato *fecha_fin* que guardamos en el componente. El método *.to_string()* nos permite garantizar que el dato que pasamos es un dato tipo String. Recordemos que una de las claves de Rust es que permite un tipado fuerte.

### Método *fin_boda*

```rust
pub fn actualizar_nft(&mut self, nft_boda: BucketRef) {
    let mut datos_nft: DatosNft = self.boda_nft_def.get_non_fungible_data(&nft_boda.get_non_fungible_key());
    datos_nft.fecha_fin = self.fecha_fin.to_string();

    self.auth_nft.authorize(|auth| self.boda_nft_def.update_non_fungible_data(&nft_boda.get_non_fungible_key(), datos_nft, auth));
}
```
Si el el componente ha actualizado el dato de fecha_fin de la boda, los NFT estarán desactualizados, con este método creamos un mecanismo por el cual alguien que quiera actualizarlos solo lo debe de enseñar su NFT para que el componente lo actualice. Recuerda que solo podremos actualizar los datos mutables de los recursos no fungibles.

```rust
let mut datos_nft: DatosNft = self.boda_nft_def.get_non_fungible_data(&nft_boda.get_non_fungible_key());
```

Primero extraemos los datos (get_non_fungible_data) del NFT para ello utilizaremos la referencia (nft_boda: BucketRef) del NFT que han pasado al realizar la llamada al método y utilizaremos su identificador (get_non_fungible_key()) . Guardamos los datos extraidos dentro de una variable mutable *datos_nft*.

```rust
 datos_nft.fecha_fin = self.fecha_fin.to_string();
```

Actualizamos el dato de *fecha_fin* que es el único mutable.

```rust
self.auth_nft.authorize(|auth| self.boda_nft_def.update_non_fungible_data(&nft_boda.get_non_fungible_key(), datos_nft, auth));
```
Finalmente actualizamos el NFT, para ello como ya hemos visto en unidades anteriores primero necesitamos la autorización (auth) que se encuentra guardada dentro del contenedor *auth_nft*. Luego gracias al método de actualización *update_non_fungible_data* que aplicamos sobre la definición del NFT *boda_nft_def* podremos actualizar los datos del NFT que le indiquemos gracias a su indentificador único. 

### Método *sacar_todo*

```rust
pub fn sacar_todo(&mut self, autorizacion: Bucket) -> (Bucket, Bucket) {
    assert_eq!(self.admin_def, autorizacion.resource_def(), "Debe entrar el recurso: Admin Lista de Boda");
    assert!(autorizacion.amount() == self.aprobacion, "No tienes el poder de consenso");
    (autorizacion, self.regalos.take_all())
}
```

Este método lo hemos actualizado incorporando un sencillo sistema de consenso o validación para que se solo quien posea las dos insignias creadas pueda retirar los fondos. Nota: La idea es que las insignias se repartan entre los contrayentes y así solo si los dos están de acuerdo pueden retirar los fondos. 

```rust
 assert_eq!(self.admin_def, autorizacion.resource_def(), "Debe entrar el recurso: Admin Lista de Boda");
```
Gracias al marco *assert_eq!* comprobamos si las definiciones coinciden. La definición de la insignia *admin_def* con la que nos pasan en la llamada *autorizacion.resource_def()*.

```rust
 assert!(autorizacion.amount() = self.aprobacion, "No tienes el poder de consenso");
```
Aquí validamos la cantidad de unidades de la insignia de administrador que está pasando y comparamos con la cantidad que hemos declarado en el componente. En este caso hemos creado solo dos insignias y el consenso mínimo es del 100%.


### Código completo:
```rust
use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct DatosNft {
    mensaje_detalle: String,
    fecha_boda: String,
    #[scrypto(mutable)]
    fecha_fin: String
}

blueprint! {
    struct ListaBoda {
        registro: HashMap<u128, (String,String,Decimal)>,
        regalos: Vault,
        boda_nft_def: ResourceDef,
        auth_nft: Vault,
        fecha_boda: String,
        fecha_fin: String,
        admin_def: ResourceDef,
        aprobacion: u8
    }

    impl ListaBoda {
        pub fn new(descripcion_nft: String, fecha_boda: String) -> (Bucket, Component) {

            let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Admin Lista de Boda")
                .initial_supply_fungible(2);

            let auth_nft = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                        .metadata("name", "Autorizacion para crear nft boda")
                        .initial_supply_fungible(1);

            let boda_nft_def = ResourceBuilder::new_non_fungible()
                                .metadata("name", descripcion_nft)
                                .flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE)
                                .badge(auth_nft.resource_def(), MAY_MINT | MAY_CHANGE_INDIVIDUAL_METADATA)
                                .no_initial_supply();
            
            let comp = Self {
                registro: HashMap::new(),
                regalos: Vault::new(RADIX_TOKEN),
                boda_nft_def,
                auth_nft: Vault::with_bucket(auth_nft),
                fecha_boda: String::from(fecha_boda),
                fecha_fin: String::new(),
                admin_def: admin.resource_def(),
                aprobacion: 2
            }
            .instantiate();

            (admin, comp)
        }

        pub fn nuevo(&mut self, nombre: String, comentario: String, mut regalo: Bucket) -> (Bucket, Bucket) {
            let mensaje = "Gracias por asistir a nuestra boda ".to_string() + &nombre;

            self.registro.insert(Uuid::generate(),(nombre,comentario, regalo.amount()));

            self.regalos.put(regalo.take(regalo.amount()));

            let nft_boda: Bucket = self.auth_nft.authorize(|badge| {
                self.boda_nft_def.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), DatosNft{
                    mensaje_detalle: mensaje,
                    fecha_boda: self.fecha_boda.to_string(),
                    fecha_fin: String::new()
                }, badge)
            });

            (regalo, nft_boda)
        }

        #[auth(admin_def)]
        pub fn fin_boda(&mut self, fin: String) {
            self.fecha_fin = fin.to_string();
        }

        pub fn actualizar_nft(&mut self, nft_boda: BucketRef) {
            let mut datos_nft: DatosNft = self.boda_nft_def.get_non_fungible_data(&nft_boda.get_non_fungible_key());
            datos_nft.fecha_fin = self.fecha_fin.to_string();

            self.auth_nft.authorize(|auth| self.boda_nft_def.update_non_fungible_data(&nft_boda.get_non_fungible_key(), datos_nft, auth));
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

        pub fn sacar_todo(&mut self, autorizacion: Bucket) -> (Bucket, Bucket) {
            assert_eq!(self.admin_def, autorizacion.resource_def(), "Debe entrar el recurso: Admin Lista de Boda");
            assert!(autorizacion.amount() == self.aprobacion, "No tienes el poder de consenso");
             (autorizacion, self.regalos.take_all())
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
resim call-function $pack ListaBoda new "Boda de Miriam y Esther" "Dos de febrero de 2020"   
set comp [dirección del componente]
```
7. Probar método *nuevo*
```rust
resim call-method $comp nuevo "Emilio Bitcoin" "Que boda mas bonita, os deseo muchos hijos..." 1000,$xrd
// Ojo que cuando pasamos un dato de tipo string y tiene mas de una palabra debemos entrecomillarlo.
// Podemos comprobar el NFT que nos han regalado
resim show $acct
```
8. Probar método *leer_registros*
```rust
resim call-method $comp leer_registros
```
9. Sacar los regalos del componente a la billetera que lo ejecuta
```rust
resim call-method $comp sacar_todo 2,[definición del recurso fungible Admin Lista de Boda]
```
10. Comprobamos que hemos recibido los xrd
```rust
resim show $acct
```
11. Actualizamos la fecha de fin de boda
```rust
resim call-method $comp fin_boda "22 de febrero de 2025" 1,[definición del recurso fungible Admin Lista de Boda]
```
12. Actualizamos el NFT
```rust
resim call-method $comp actualizar_nft 1,[definición del nft que nos han regalado]
```
:::

😎 ¿Qué tal? ¿Como te sientes después de 10 unidades de aprendizaje? Yo solo puede decir que para mi ha sido un verdadero honor  haberte ayudado a emprender una ruta fantástica. El camino de la programación es un sendero de continuo aprendizaje de mucha practica e ilusión por logra vencer los retos que nos marcamos. Hoy puedes decir que conoces los fundamentos de Radix y Scrypto, puedes decir que puedes leer, ejecutar e incluso escribir tus propios componentes. Hoy puedes decir que te has iniciado en la programación funcional de la mano de Rust. De verdad que me emociona pensar que esta academia ha sido tu punto de partida para amar como yo la programación y el diseño de tecnología aplicada. 

¿Te atreves? a seguir aprendiendo, e incluso realizando tus primeras prácticas. Solo cuando empieces por ti mismo a realizar tus propios algoritmos podrás decir que eres un programador junior en Scrypto. 

¿Te apetece? Pues te invito a resolver una pequeña [prueba de nivel](/ruta/basico/unidad10.md) que demuestre que has asimilado algunos conocimientos de programación y Scrypto. Solo así podrás acceder al nivel intermedio donde empezará lo verdaderamente emocionante!!!! No te arrepentirás...

Hagas lo que hagas... gracias, mil gracias por llegar hasta aquí. Solo las personas verdaderamente valientes terminan lo que empiezan y tu, sin duda, eres uno de ellos. De verdad gracias, para mi fue un placer.


