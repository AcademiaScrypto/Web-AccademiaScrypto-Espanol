# Tipo Datos
Los *tipos de datos* , en programación, suelen ir de la mano con las **operaciones aritméticas**, y Rust no es diferente.
## 🔢 Números
Tenemos tipos de *enteros* como **i32** o **i64** y *número de punto flotante* como **f64**.

<div class="alert alert-dark" role="alert">
  <b>¿Qué es un número entero?</b>
  <p>Los números enteros son aquellos que no contienen decimales, pueden ser positivos o negativos además del cero.</p>
  <hr>
  <b>¿Qué es un número de punto flotante?</b> 
  <p>Viene a ser un <i>número real</i>, 😅, es un numero con decimales básicamente.</p>
  <i>Nota: la explicación puede ser algo más compleja, pero de momento quedate con esto.</i>
</div>

Tenga en cuenta que cuando declara variables, puede elegir especificar el tipo de datos indicándolo explícitamente en la declaración.  

Por ejemplo: si queremos almacenar una variable que contenga un entero sin signo de tamaño 8 bits, tenemos que hacer algo como lo siguiente:  
```
fn main() {
  let age: u8 = 18;
  println!("You are {} years old.", age)
}
```
*Nota: esto es aplicable a otros tipos de datos*
::: tip
- Si acabara de escribir *let age = 18;*, Rust habría inferido que el tipo de edad era del tipo i32. Aunque la inferencia simplifica las cosas, no siempre es la mejor manera de programar. 
:::

## ✔️❌ Booleanos
Tambien tenemos los datos de tipo **bool** que puede contener los valores *true* o *false*.

## 🔠 Cadenas, & str y caracteres
El dato de tipo **string**, **&str** y **char** nos sirven para almacenar textos.

::: tip 
- El tipo carácter (*char*) se escribe con comillas simples, ejemplo 'C'
- Las cadenas de datos (*string*) se implementan como una coleción de bytes. 
- **&str** se conoce como segmento de cadena o literal de cadena, **inmutable**
:::
Ejemplo:
```
let s = "Hacia la ciencia de datos";
```
aquí Rust infiere que *s* es **&str** en cambio con el siguiente comando lo tomara como una cadena tipo **string**:
```
let s = String :: from ("Hacia la ciencia de datos");
```
## 👨‍👩‍👧‍👦 Colecciones
**Tupple**, **Array**, **Vector** y **hash map** como colecciones de datos que a su vez se convierten en si mismo en un dato.
- **Array** tamaño fijo (acceso más rápido, ¡ya hablaremos de esto!)
```
let mi_array = [1, 2, 3];
```
- **Tupple** se usa para almacenar diferentes tipos de datos en una colección por ejemplo:
```
let mezcla: (i32, f64, char) = (7, 3.141592, 'K');
let pi = mezcla.1;
println!("El numero pi es aproximadamente igual a {}... ", pi);
```
- **Vector** es una colección que puede **aumentar de tamaño**, solo puede contener un tipo de dato. 
```
sea ​​v = vec! [1, 2, 3];
```
**Hash map**: almacena un mapeo de claves de tipo *k* a valores de tivo *v*.
```
fn main() {
  use std::collections::HashMap;

  let mut webs = HashMap::new();

  webs.insert(String::from("Estado"), "https://status.radixdlt.com/");
  webs.insert(String::from("Explorador"), "https://explorer.radixdlt.com/");
  webs.insert(String::from("Wallet"), "https://wallet.radixdlt.com/");
  webs.insert(String::from("Radix Scrypto Github"), "https://github.com/radixdlt/radixdlt-scrypto");

  }
```
## 🔣 Enum
 **Enum** un tipo de dato que juega un papel muy importante en Rust, en realidad podemos crear una lista de valores posibles como por ejemplo:  
```
enum Genero {
    Hombre, Mujer
}
```
***
## ♾️ Otros tipos
¡Como no...! también existen **structs**, **methods** y **associated functions** que es como un tipo de clases en la programación orientada a objetos. **Trait** es como una interface de GoLang.


::: tip Structs
- En la sintaxis de los **blueprints** utilizamos **structs** 😉
:::

::: warning Resumen
- Tip: ¡El uso de algunos de estos tipos se aprende mejor a través de ejemplos y experimentando!, ya iremos viendo ejemplos, de momento, quédate con una idea general de los tipos de datos que existen en Rust.
:::

