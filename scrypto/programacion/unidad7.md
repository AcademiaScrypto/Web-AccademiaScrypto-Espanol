# DAO - Sistema de recompensas a miembros y bloqueo

::: warning
- Esta unidad **NO** está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Soon Video Demostración]()
- [Código GitHub]()
:::

En la [unidad anterior](/scrypto/programacion/unidad6.md) comenzamos con la creación de una [DAO](/fundamentos/blockchain/dao.md) y pudimos codificar tanto al función constructora como una para el ingreso de nuevos miembros. Hoy vamos a crear dos nuevos métodos, uno para recompensar a los afiliados y otra para *banear* afiliados.

### Análisis
**Problema/Análisis:** Nos gustaría que nuestra DAO pudiera otorgar puntos a los miembros a cambio de XRD, para ir teniendo liquidez, estos puntos equivaldrán 1 a 1 con XRD, la idea es que un miembro pueda añadir liquidez al DAO y sea recompensado con tantos puntos como XRD haya añadido al tesoro de la DAO. Además nos gustaría que miembros con muchos puntos, al menos 10000, tengan el poder de bloquear a otros afiliados. 

### Diseño
![Diagramas](./img/dao2_diagrama.png)

Al menos dos nuevos métodos: uno para recoger los XRD, guardarlos en el tesoro del DAO y seguidamente convertir esos XRD en puntos que guardaremos dentro del NFT. Y otro método para permitir a aquellos afiliados que tengan más de 10000 puntos puedan bloquear a un afiliado.


### Programación

### NFT Estructura Datos:

Con el fín de guardar en los Nft los *puntos* obtenidos gracias a las aportaciones y saber si esta bloqueado o no, incluiremos en la estructura de los datos del Nft dos nuevos campos *mutables* de la siguiente manera:

```rust
#[derive(NonFungibleData)]
struct DatosAfiliado {
    nombre: String,
    #[scrypto(mutable)]
    puntos: Decimal,
    #[scrypto(mutable)]
    bloqueado: bool
}
```
Para declarar que un dato es mutable en el NFT solo hemos de agregar antes del dato el siguiente macro:

```rust
 #[scrypto(mutable)]
```

### La estructura:

```rust
struct SistemaAfiliacion {
    minteador: Vault,
    def_afiliado_nft: ResourceDef,
    num_afiliado: u128,
    aportaciones: Vault,
}
```

En este caso en la estructura del componente solo vamos a agregar un nuevo contenedor permanente, *Vault*, para guardar las aportaciones de los afiliados que se convertirán en el tesoro del DAO.

### Función constructora *new*

Al cambiar la estructura del componente es necesario también retocar la función constructora, en este caso inicializar el contenedor *aportaciones* como un nuevo contenedor de Radix Token.

Nuevas funcionalidades al nft:
```rust
.flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE)
.badge(minteador.resource_def(), MAY_MINT | MAY_CHANGE_INDIVIDUAL_METADATA)
```

Como vamos a modificar los datos del NFT vamos a otorgar esa propiedad al non fungible incluyendo una bandera tipo: **INDIVIDUAL_METADATA_MUTABLE**. Y por tanto también restringiremos esa propiedad a la insignia *minteador*: **MAY_CHANGE_INDIVIDUAL_METADATA**

```rust
pub fn new() -> Component {
    
    let minteador = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                    .metadata("name", "Autorización mintear nuevos NFT")
                    .initial_supply_fungible(1);

    let def_afiliado = ResourceBuilder::new_non_fungible()
                        .metadata("name", "Afiliado DAO")
                        .flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE)
                        .badge(minteador.resource_def(), MAY_MINT | MAY_CHANGE_INDIVIDUAL_METADATA)
                        .no_initial_supply();

    Self {
        minteador: Vault::with_bucket(minteador),
        def_afiliado_nft: def_afiliado,
        num_afiliado: 0,
        aportaciones: Vault::new(RADIX_TOKEN)
    }
    .instantiate()
}
```

### Método *aportaciones*

```rust
#[auth(def_afiliado_nft)]
pub fn aportaciones(&mut self, aportacion: Bucket) {
    let puntos: Decimal = aportacion.amount();
    self.aportaciones.put(aportacion);

    let mut nft_datos : DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&auth.get_non_fungible_key());
    assert!(!nft_datos.bloqueado, "¡Estas bloqueado en la DAO!");

    nft_datos.puntos += puntos;

    self.minteador.authorize(|badge| {
        self.def_afiliado_nft.update_non_fungible_data(&auth.get_non_fungible_key(), nft_datos, badge);
    });

    info!("Gracias por la aportación,  has recibido {} puntos en el DAO", puntos);
}
```
Antes de declara el método, estamos agregando previamente, un macro de autenticación (auth) donde indicamos que solo podrán entrar aquellos que tengan un NFT como afiliados del DAO, recuerda que *def_afiliado_nft* guarda la información de los NFT que acuñamos, pero los NFT físicamente se guarda en las cuentas. En esta macro le estamos diciendo que para poder ejecutar el método *aportaciones* nos tienen que enseñar un NFT acuñado por *def_afiliado_nft*.

```rust
#[auth(def_afiliado_nft)]
```

Seguidamente guardamos el número de XRD que nos han mandado como *aportacion* en la variable *puntos*, utilizamos la función *[amount()](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Bucket.html#method.amount)* que nos devuelve un tipo decimal con la cantidad de recurso en un *backet* o *vault*. 

```rust
let puntos: Decimal = aportacion.amount();
```

::: tip
- Recuerda que la palabra clave **let** nos permite declarar una variable. Para que su valor pueda ser modificado recuerda en establecer la partícula **mut**.
:::

Ahora que ya conocemos el número de puntos, vamos a guardar los XRD aportados en el tesoro (Vault) del DAO, para ello usamos la función **[put()](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Bucket.html#method.put)* que literalmente toma, en este caso, toma el [bucket](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Bucket.html) *aportacion* y lo introduce en el [vault](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.Vault.html) *aportaciones*.

```rust
self.aportaciones.put(aportacion);
```
Una vez hemos guardado el número de puntos y trasladado los XRD a la bóveda del DAO, vamos a tomar los datos del NFT que hemos presentado como autentificación de afiliados al DAO en el macro inicial. Los datos los vamos a guardar en una variable mutable que hemos designado arbitrariamente como *nft_datos*. 

```rust
let mut nft_datos : DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&auth.get_non_fungible_key());
```
La variable *nft_datos* la declaramos de tipo **DatosAfiliado** que es como hemos llamado a la estructura (struct) de datos del NFT. Dicha variable la llenamos con los datos del NFT que hemos pasado en el proceso de [*auth*](https://radixdlt.github.io/radixdlt-scrypto/scrypto/prelude/attr.auth.html) y del que tomamos la id con la función **[get_non_fungible_key()](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.BucketRef.html#method.get_non_fungible_key)**. Para tomar los datos del NFT, del que ya conocemos su id, utilizamos la función **[get_non_fungible_data()](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.ResourceDef.html#method.get_non_fungible_data)**

Ahora con los datos del NFT en la variable *nft_datos* ya podemos preguntar por ejemplo si el NFT recoge como bloqueado al afiliado, para ello usaremos el macro de Rust **[assert!](https://doc.rust-lang.org/std/macro.assert.html)** que evaluará la expresión, y si es *true* interrumpirá el método y devolverá un mensaje de error que como ves podemos personalizar:

```rust
assert!(!nft_datos.bloqueado, "¡Estas bloqueado en la DAO!");  
```

Si el macro anterior al ser evaluado devuelve *false* pasaremos a actualizar los puntos del afiliado: 

```rust
nft_datos.puntos += puntos;

self.minteador.authorize(|badge| {
    self.def_afiliado_nft.update_non_fungible_data(&auth.get_non_fungible_key(), nft_datos, badge);
});
```

Primero sumamos los puntos a los que ya tuviera en afiliado en el NFT con el operador de asignación aritmético **+=**. Y seguidamente actualizamos los datos del NFT con la función **[update_non_fungible_data()](https://radixdlt.github.io/radixdlt-scrypto/scrypto/resource/struct.ResourceDef.html#method.update_non_fungible_data)** a la que le pasamos los siguientes argumentos: identificador del nft, datos actualizados, y autorización. El procedimiento es parecido al acuñado. 

Finalmente si todo el proceso ha sido exitoso lanzaremos el marco **[info!()](https://radixdlt.github.io/radixdlt-scrypto/scrypto/macro.info.html)** para devolver un mensaje de logro. 

```rust
info!("Gracias por la aportación,  has recibido {} puntos en el DAO", puntos);
``` 

### Método *bloqueo*

```rust
#[auth(def_afiliado_nft)]
pub fn bloqueo(&mut self, nft_id: NonFungibleKey) {
    let nft_datos: DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&auth.get_non_fungible_key());
    assert!(!nft_datos.bloqueado, "¡Estas bloqueado en la DAO!");
    assert!(nft_datos.puntos >= 1000.into(), "No tienes suficientes puntos para bloquear a otro afiliado");

    let mut nft_datos_bloqueo: DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&nft_id);
    nft_datos_bloqueo.bloqueado = true;
    self.minteador.authorize(|badge| {
        self.def_afiliado_nft.update_non_fungible_data(&nft_id, nft_datos_bloqueo, badge);
    });
}
```

```rust 
#[auth(def_afiliado_nft)]
```
Al igual que en método anterior, en este también utilizamos la **[autorización (auth)](https://radixdlt.github.io/radixdlt-scrypto/scrypto/attr.auth.html)** para garantizar que la persona que ejecuta este método es alguien perteneciente al DAO y que pueda demostrarlo con su NFT de afiliado. 

```rust
pub fn bloqueo(&mut self, nft_id: NonFungibleKey)
```
Este método necesita como argumento, además del *self* o datos permanentes de la estructura, un tipo de [dato NonFungibleKey](https://radixdlt.github.io/radixdlt-scrypto/scrypto/types/struct.NonFungibleKey.html) donde pasaremos al método el identificador del NFT que queremos bloquear. 

```rust
let nft_datos: DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&auth.get_non_fungible_key());
```

Al igual que en método anterior tomamos los datos del nft de la persona que ejecuta el método y pasa su NFT como acreditación. Esto lo hacemos, en este caso, primero para saber si es un usuario bloqueado y entonces no le dejaremos seguir:

```rust
assert!(!nft_datos.bloqueado, "¡Estas bloqueado en la DAO!");
```

::: tip
- Para negar utilizamos **!**, es decir estamos preguntando si "¿no es true?"
:::

y segundo cerciorarnos de que la persona posea, al menos, suficientes puntos de afiliado como para poder bloquear a otro:

```rust
assert!(nft_datos.puntos >= 10000.into(), "No tienes suficientes puntos para bloquear a otro afiliado");
```

en ambos casos utilizamos el macro **assert!()** que ya vimos en el método anterior. 

Si finalmente se han pasado todos los filtros mencionados anteriormente como no estar bloqueado y poseer mas de 10000 puntos, tomaremos los datos del nft a bloquear y lo actualizaremos sus datos mutables de bloqueo a *true*:

```rust
let mut nft_datos_bloqueo: DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&nft_id);
nft_datos_bloqueo.bloqueado = true;
self.minteador.authorize(|badge| {
    self.def_afiliado_nft.update_non_fungible_data(&nft_id, nft_datos_bloqueo, badge);
});
```

Este procedimiento es parecido de actualización de los datos mutables de un nft es parecido al que ya utilizamos en el método anterior. Reseñar que podemos hacerlo ya que poseemos la autorización del *minteador* que se guarda en el propio componente para realizar estas acciones y que pasamos a la función *update_non_fungible_data* a través de la variable *badge*.

### Código completo:
```rust
use scrypto::prelude::*;

#[derive(NonFungibleData)]
struct DatosAfiliado {
    nombre: String,
    #[scrypto(mutable)]
    puntos: Decimal,
    #[scrypto(mutable)]
    bloqueado: bool
}

blueprint! {
    struct SistemaAfiliacion  {
        num_afiliado: u128,
        def_afiliado_nft: ResourceDef,
        minteador: Vault,
        aportaciones: Vault
    }

    impl SistemaAfiliacion  {
        
        pub fn new() -> Component {
            let minteador: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Autorización mintear nuevos NFT")
                .initial_supply_fungible(1);

            let def_afiliado = ResourceBuilder::new_non_fungible()
                .metadata("name", "Afiliado DAO")
                .flags(MINTABLE | INDIVIDUAL_METADATA_MUTABLE)
                .badge(minteador.resource_def(), MAY_MINT | MAY_CHANGE_INDIVIDUAL_METADATA)
                .no_initial_supply();

            Self {
                minteador: Vault::with_bucket(minteador),
                def_afiliado_nft: def_afiliado,
                num_afiliado: 0,
                aportaciones: Vault::new(RADIX_TOKEN)
            }
            .instantiate()
        }

        
        pub fn afiliarse_dao(&mut self, nom: String) -> Bucket {
            self.num_afiliado += 1;
            
            self.minteador.authorize(|insignia| {
                self.def_afiliado_nft.mint_non_fungible(&NonFungibleKey::from(self.num_afiliado), DatosAfiliado{
                    nombre: nom , puntos: Decimal::zero(), bloqueado: false}, insignia)
            })
        }
        
        #[auth(def_afiliado_nft)]
        pub fn aportaciones(&mut self, aportacion: Bucket) {
            let puntos: Decimal = aportacion.amount();
            self.aportaciones.put(aportacion);

            let mut nft_datos : DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&auth.get_non_fungible_key());
            assert!(!nft_datos.bloqueado, "¡Estas bloqueado en la DAO!");

            nft_datos.puntos += puntos;

            self.minteador.authorize(|badge| {
                self.def_afiliado_nft.update_non_fungible_data(&auth.get_non_fungible_key(), nft_datos, badge);
            });

            info!("Gracias por la aportación,  has recibido {} puntos en el DAO", puntos);
        }

        #[auth(def_afiliado_nft)]
        pub fn bloqueo(&mut self, nft_id: NonFungibleKey) {
            let nft_datos: DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&auth.get_non_fungible_key());
            assert!(!nft_datos.bloqueado, "¡Estas bloqueado en la DAO!");
            assert!(nft_datos.puntos >= 1000.into(), "No tienes suficientes puntos para bloquear a otro afiliado");

            let mut nft_datos_bloqueo: DatosAfiliado = self.def_afiliado_nft.get_non_fungible_data(&nft_id);
            nft_datos_bloqueo.bloqueado = true;
            self.minteador.authorize(|badge| {
                self.def_afiliado_nft.update_non_fungible_data(&nft_id, nft_datos_bloqueo, badge);
            });
        }
    }
}
```

### Compilación y ejecución
A estas alturas seguro que ya sabes publicar el package, instanciar el component y llamar a las funciones pasando un parámetro.

::: danger Recuerda 
- Puedes sobre-escribir un package y modificarlo sin necesidad de cambiar de dirección del package:
```rust
resim publish . --address $package 
```
:::

En esta ocasión mencionaros que para realizar pruebas y ejecutar el componente deberíais crear más de una cuenta, esto es sencillo solo hay que ejecutar de nuevo la instrucción:

```
resim new-account
```
Y guardarla en una nueva variable por ejemplo:

```
set account2 [numero de cuenta]
```

**Recuerda** guardar también la información de la cuenta como la dirección de sus XRD y el NFT de afiliado para despues utilizarlo.

Para cambiar de una cuenta a otra dentro del simulador has de ejecutar la siguiente instrucción:

```
resim set-default-account [account2_address] [account2_pubkey]
```

Esta instrucción establece una cuenta como principal (o por defecto), y no solamente necesitaremos la dirección sino también la llave pública (pubkey) la cual podemos conseguir en el momento de crear la cuenta: 

```rust 
PS C:\Users\Andres\radixdlt-scrypto\dao> resim new-account
A new account has been created!
Account address: 022f189e2f5d2ebf08d5d9d9c0ed0503748b8922bbf1a86c18b5c2
Public key: 007902699be42c8a8e46fbbb4501726517e86b22c56a189f7625a6da49081b2451
```
Y guardala como hicimos hasta ahora:

```
set pubkey2 04007902699be42c8a8e46fbbb4501726517e86b22c56a189f7625a6da49081b2451
```
::: details Solo para aquellos (tipo Emilio 🤪) que no quieren pensar!!!
1. Limpiar el simulador
```rust
resim reset
```
2. Crear un Package
```
scrypto new-package dao_mejorado
```
3. Crear una cuenta (recuerda copiar la dirección de los XRD de tu cuenta)
```rust
resim new-account
set acct [Address de la cuenta que acabamos de crear]
set pub [Guardamos la clave pública de esta cuenta]
```
4. Copiar o escribir el código (recuerda guardar ctrl + s)
- Recuerda guardar el código de este ejercicio dentro del archivo lib.rs que has creado en la carpeta *\radixdlt-scrypto\dao\src\lib.rs*
5. Publicar y guardamos la dirección del Package
```rust
resim publish .
set pack [New Package Reference]
```
6. Instanciar componente (recuerda que en este caso hay que añadir el argumento del precio) y guardar la dirección del componente.
```rust
resim call-function $pack SistemaAfiliacion new
set comp [dirección del componente]
```
7. Probar método *afiliarse_dao*
```rust
resim call-method $comp afiliarse_dao "Emilio Bitcoin"
// Ojo que cuando pasamos un dato de tipo string y tiene mas de una palabra debemos entrecomillarlo.
```
8. Comprobar el nft de afiliado
```rust
resim show $acct
```
9. Guardamos la definición del xrd para realizar aportaciones
```rust
resim show $acct
set xrd 030000000000000000000000000000000000000000000000000004
```
10. Mandamos aportaciones al DAO que se convertirán en puntos dentro del nft
```rust
resim call-method $comp aportaciones 1000,$xrd 1,0399d3f4678fbf0ec6abb57bb17af7ddcc48ce1370e65eb99f8e13
// recuerda que para pasar una autorización lo hacemos al final, donde 1 es la unidad a pasar el resto es la definición del non fungible
```
11. Vemos los cambios en nuestro nft
```rust
resim show $acct
```
12. Para probar el bloqueo podemos crear una nueva cuenta y definirla por defecto para usarla
```rust
resim new-account
set acct2 xxxxxxxxxxxxxxxx
set pub2 xxxxxxxxxxxxxx
resim set-default-account $acct2 $pub2
resim call-method $comp afiliarse_dao "Antonio Mister BTC"
resim show $acct2
set id2=00000000000000000000000000000002
```
13. Volvemos a definir $acct como cuenta por defecto para poder bloquear $acct2
```rust
resim set-default-account $acct $pub
resim call-method $comp bloqueo $id2 1,xxxxxxxxxxxxxxxxxxxxxx
resim show $acct2
```
:::

😎 ¿Qué tal?, ¡seguro no fue facil llegar hasta aquí! pero sabes ya sabes muchas muchas cosas que antes ni imaginabas. ¡Has podido crear una DAO! cierto, es muy sencilla pero el límite lo pones tú. En pocos unidades más habrás terminado el nivel básico y seras todo un desarrollador *Junior* de contratos inteligentes con Scrypto. No te rindas...!!!
