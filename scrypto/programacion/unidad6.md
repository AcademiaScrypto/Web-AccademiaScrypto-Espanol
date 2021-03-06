# Creaci贸n de DAO - Nuevos miembros

::: warning
- Esta unidad est谩 actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualizaci贸n](/scrypto/instalacion/actualizacion.md)
:::

::: tip 馃摵
- [Pronto Video Demostraci贸n]()
- [C贸digo GitHub](https://github.com/AcademiaScrypto/Web-AccademiaScrypto-Espanol/blob/master/scrypto/codigo/unidad7.rs)
:::

Durante las siguientes unidades vamos a crear una DAO (Organizaci贸n aut贸noma descentralizada), que sin duda durante 2022 est谩n llamadas a ser tendencia y seguro que han llegad1o para quedarse entre nosotros. Aprender a crear DAOs puede tener un gran potencial. 隆Comencemos!

### An谩lisis
**Problema:** Deseamos crear una DAO donde los afiliado van a poder darse de alta sin ning煤n tipo de autorizaci贸n. 

**An谩lisis:** Al pedirnos una DAO ya estamos dando por supuesto que vamos a crear una dApp. Para ello vamos a crear un *Componente*, dentro de la red de Radix, que nos va a permitir dar un *no fungible* a cada nuevo afiliado para que pueda demostrar su pertenencia y en un futuro identificarse dentro de la DAO.

### Dise帽o:
![Diagramas](./img/diagramas_unidad6.png)

Dos diagramas: uno para la funci贸n constructora y el segundo para la primera funcionalidad que nos solicitan y es la de poder registrarse, cualquiera, al DAO retornando un *no fungible* como acreditaci贸n de su pertenencia al DAO. *(Nota: en cuanto m谩s lenguaje m谩s natural expresemos en estos diagramas mejor, yo a veces peco de incluir t茅rminos t茅cnicos propios del lenguaje o plataforma que utilice)*

### Programaci贸n:

### NFT Estructura Datos:

Como ya vimos en la [unidad anterior](/scrypto/programacion/unidad5.md) *Scrypto* pone a disposici贸n una macro para poder crear una estructura de datos, mutables e inmutables, que luego podremos incorporar a nuestros *no fungible*:

```rust
#[derive(NonFungibleData)]
pub struct DatosAfiliado {
    nombre: String
}
```

En este caso hemos llamado a la estructura: *DatosAfiliado* (recuerda, es algo arbitrario) y de momento solo le hemos implementado un dato de tipo *String* (Cadena de caracteres) para guardar el nombre del afiliado (ser谩 un dato inmutable).

### La estructura:

Vamos a necesitar crear un contador para guardar el n煤mero de afiliado, lo normal es que esta vez sea un n煤mero legible y no uno aleatorio, y que a su vez nos har谩 de identificador del NFT. Para poder utilizar como identificadores [*NonFungibleId*](https://radixdlt.github.io/radixdlt-scrypto/scrypto/engine/types/struct.NonFungibleId.html#) tenemos que crear un dato de tipo u32.

::: tip
|DATA TYPE   |MIN    |MAX |
| ---------- |-------|----|
|u8	         |0      |255 |
|u16         |0      |65535 |
|u32         |0      |4294967295 |
|u64	     |0      |18446744073709551615 |
|u128        |0      |340282366920938463463374607431768211455 |
:::

Luego necesitaremos guardar la direcci贸n del recurso no fungible que queremos dar a cada afiliado, est谩 ser谩 de tipo ResourceAddress.

::: tip
- Gracias a la direcci贸n del recurso, ResourceAddress, podemos determinas si el no fungible que alguien nos presente es el que esper谩bamos o no. 
:::

Y de momento solo nos quedar铆a a帽adir un contenedor permanente (Vault) para guardar una *insignia* (badge) que autorice al Componente para mintear nuevos *no fungibles*. En este caso la insignia con autorizaci贸n para acu帽ar se guardara en el propio componente para que la utilice cuando sea necesario. 

```rust
struct Sistema_afiliacion {
        badge: Vault,
        direccion_nft: ResourceAddress,
        num_afiliado: u32
}
```

### Funci贸n constructora *instantiate_dao*

1. Primero vamos a crear la autorizaci贸n para crear el *no fungible*:
```rust
let badge = ResourceBuilder::new_fungible()
                .metadata("name", "Autorizaci贸n mintear nuevos NFT")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);
```
Creamos un recurso con, *ResourceBuilder*, de tipo fungible. El *name* que lo identifique, es especialmente interesante para poder identificarlo cuando realizamos *resim show*. Especificamos que no sera divisible. Y finalmente un suministro inicial de 1.

::: tip
Llamar .divisibility al crear un nuevo recurso es opcional. Si no se llama, la divisibilidad del recurso se establece en el valor predeterminado de 18.
:::

2. Creamos la definici贸n *no fungible* que vamos a entregar a cada persona que se registre en el DAO:

```rust
let afiliado = ResourceBuilder::new_non_fungible()
    .metadata("name", "Afiliado DAO")
    .mintable(rule!(require(badge.resource_address())), LOCKED)
    .no_initial_supply();
```
En este caso el recurso ser谩 de tipo no fungible, *new_non_fungible*, y tiene varios m茅todos que pueden definir sus caracter铆sticas:

-  *.metadata*: Permite dar nombre al recurso.

Seguidamente a帽adiremos al recurso una regla de autorizaci贸n para poder acu帽ar nuevos *no fungibles*:  

```rust
.mintable(rule!(require(badge.resource_address())), LOCKED)
```
En este caso la regla solo permite a la insignia (badge) que hemos acu帽ado antes y que hemos guardado en el propio Componente. E incluimos el par谩metro *LOCKED* que impide que sea modificado la regla de acu帽aci贸n para este recurso. 

Finalmente, *.no_initial_supply()*, que indica que no vamos a crear inicialmente ning煤n suministro.

3. Pasamos a la estructura del componente:
    - La insignia (badge) que hemos creado para autorizar acu帽ar nuevos NFT, como tenemos el recurso (supply 1) en una variable, *badge*, podemos utilizar el comando *with_bucket* para crea una b贸veda vac铆a permanente y llenarla con un dep贸sito inicial de recursos contenidos en un bucket.
    - Pasamos la definici贸n del recurso de NFT al componente.
    - Inicializamos la variable *num_afiliado* a cero. 

```rust
Self {
        badge: Vault::with_bucket(badge),
        direccion_nft: afiliado,
        num_afiliado: 0
    }
    .instantiate()
    .globalize()
```

### M茅todo *afiliarse_dao*

Este m茅todo permite poder registrarse en la DAO como afiliado sin ning煤n tipo de autorizaci贸n para ello, de forma totalmente libre. Solo ha de introducir un nombre de tipo string. 

El m茅todo al ejecutarse toma el nombre que se le ha indicado, a帽ade 1 al num_afiliado para generar el nuevo n煤mero y finalmente acu帽a el NFT con esos dos datos: el numero afiliado como identificador y el nombre como dato inmutable. Finalmente el m茅todo devuelve un contenedor temporal con la acreditaci贸n de afiliado al DAO en forma de NFT a la cuenta que ha desencadenado este m茅todo. 


```rust
pub fn afiliarse_dao(&mut self, nombre: String) -> Bucket {
    self.num_afiliado += 1;

    self.badge.authorize(||{
        let resource_manager = borrow_resource_manager!(self.direccion_nft);
            resource_manager.mint_non_fungible(
                &NonFungibleId::from_u32(self.num_afiliado),
                DatosAfiliado {
                    nombre: nombre
                }
            )
    })
}
```

Pasamos la autorizaci贸n para acu帽ar mediante el contenedor permanente *badge* de la siguiente manera:
```rust
self.badge.authorize(|| {
}
```
Tomamos el recurso ResourceManager a trav茅s del la siguiente macro *borrow_resource_manager!* a la que pasamos como par谩metro la direcci贸n del recurso que queremos utilizar *ResourceAddress* que hemos guardado en la variable *direccion_nft*.

```rust
let resource_manager = borrow_resource_manager!(self.direccion_nft);
```
::: tip
**ResourceManager** nos permite realizar operaciones como acu帽ar tokens adicionales, grabar tokens, actualizar metadatos, etc.
:::

Una vez tenemos el ResourceManager del recurso que queremos acu帽ar utilizamos el m茅todo *mint_non_fungible* al que pasamos los datos: identificador y los datos de la estructura que declaramos como *DatosAfiliado* que contiene el nombre del afiliado.

El tipo NonFungibleId nos permite dar al *no fungible* un identificador 煤nico. En este caso, el identificador ser谩 el n煤mero de afiliado que hemos generado y guardo en la variable *num_afiliado.

Como es un tipo u32 lo convertimos a *NonFungibleId* con el m茅todo *from_u32* que recibe como par谩metro el n煤mero de afiliado.

```rust
&NonFungibleId::from_u32(self.num_afiliado)
```
Finalmente pasamos los datos del no fungible, en este caso solo el nombre que previamente pasamos como entrada al m茅todo. 

```rust
DatosAfiliado {
    nombre: nombre
}
```

### C贸digo completo:

```rust
use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct DatosAfiliado {
    nombre: String
}

blueprint! {
    struct Afiliacion {
        badge: Vault,
        direccion_nft: ResourceAddress,
        num_afiliado: u32
    }

    impl Afiliacion {

        pub fn instantiate_dao() -> ComponentAddress {
            
            let badge = ResourceBuilder::new_fungible()
                .metadata("name", "Autorizaci贸n mintear nuevos NFT")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(1);
                
            let afiliado = ResourceBuilder::new_non_fungible()
                .metadata("name", "Afiliado DAO")
                .mintable(rule!(require(badge.resource_address())), LOCKED)
                .no_initial_supply();

            Self {
                badge: Vault::with_bucket(badge),
                direccion_nft: afiliado,
                num_afiliado: 0
            }
            .instantiate()
            .globalize()
        }

        pub fn afiliarse_dao(&mut self, nombre: String) -> Bucket {
            self.num_afiliado += 1;
        
            self.badge.authorize(||{
                let resource_manager = borrow_resource_manager!(self.direccion_nft);
                    resource_manager.mint_non_fungible(
                        &NonFungibleId::from_u32(self.num_afiliado),
                        DatosAfiliado {
                            nombre: nombre
                        }
                    )
            })
        }
    }
}
```
### Compilaci贸n y ejecuci贸n

A estas alturas seguro que ya sabes publicar el *package*, instanciar el *Component* y llamar a los m茅todos pasando un par谩metro.

::: warning Recuerda
- Antes de empezar siempre es recomendable limpiar el simulador con el comando:
```
resim reset
```
:::

::: details Solo para aquellos (tipo Emilio 馃お) que no quieren pensar!!!
1. Limpiar el simulador
```rust
resim reset
```
2. Crear un Package
```
scrypto new-package dao
```
3. Crear una cuenta (recuerda copiar la direcci贸n de los XRD de tu cuenta)
```rust
resim new-account
set acct [Address de la cuenta que acabamos de crear]
```
4. Copiar o escribir el c贸digo (recuerda guardar ctrl + s)
- Recuerda guardar el c贸digo de este ejercicio dentro del archivo lib.rs que has creado en la carpeta *\radixdlt-scrypto\dao\src\lib.rs*
5. Publicar y guardamos la direcci贸n del Package
```rust
resim publish .
set pack [New Package Reference]
```
6. Instanciar componente (recuerda que en este caso hay que a帽adir el argumento del precio) y guardar la direcci贸n del componente.
```rust
resim call-function $pack Afiliacion instantiate_dao
set comp [direcci贸n del componente]
```
7. Probar m茅todo *afiliarse_dao*
```rust
resim call-method $comp afiliarse_dao "Antonio Bitcoin"
// Ojo que cuando pasamos un dato de tipo string y tiene mas de una palabra debemos entrecomillarlo.
```
8. Comprobar el nft de afiliado
```rust
resim show $acct
```
:::

::: danger Importante:
- Soy muy consciente de que hay muchas que no has entendido, 隆TRANQUILO!, no te rindas, las entender谩s... 馃槈
:::