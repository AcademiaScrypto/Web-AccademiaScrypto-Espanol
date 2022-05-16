# Estructuras

Podriamos decir que las estructuras (y luego veremos que también las enumeraciones) son uno de los componentes básicos de Rust. En realidad las estructuras nos permiten almacenar en un solo contenedor datos relacionados. Dicha estructura permiten crear tipos de datos más complejos. Imagina que quieres guardar las coordenadas *x* e *y*, lo podriamos hacer de la siguiente manera:

```
let x: i32 = 0;
let y: i32 = 0;
```
pero con las estructuras (*struct*) podemos combinar estos datos para que formen uno unificado:

```
struct Punto {
    x: i32,
    y: i32,
}
```
::: tip
Por convenio las *struct* comienzan con una letra mayuscula y son *camel case*: Ej. PuntoEnElEspacio, y no Punto_En_El Espacio
:::

Este ejemplo de estructura 'Punto' podriamos utilizarlo de la siguiente manera:

```
fn main() {
    let mut punto = Punto { x: 0, y: 0 };

    punto.x = 5;

    println!("El origen esta en ({}, {})", punto.x, punto.y);
}
```

Si te fijaste usamos la sintaxis estilo *clave: valor* para asignar valor a cada campo de la estructura. Algo interesante es que el orden no necesita ser el mismo que en la declaración de la estructura. Debido a que tenemos nombres de campos, podemos acceder a ellos a través de la notación puntos: Ej. *origen.x*

::: tip
Rust no soporta mutabilidad de campos a nivel de lenguaje. La mutabilidad es una propiedad del enlace a variable, no de la estructura en si misma.
:::

Debido a que la mutabilidad en Rust es una propiedad de las variables en el ejemplo anterior definimos la variable *punto* con la particula *mut*, gracias a esto pudimos cambiarle el valor *(línea 4 del ejemplo anterior)*

### Tupla estructuras (Tuple structs)

Rust posee otro tipo de datos que es como un híbrido entre una *tupla* y una *struct*. Las tupla estructuras poseen un nombre, pero sus campos no:

```
struct Color(i32, i32, i32);
struct Punto(i32, i32, i32);

let negro = Color(0, 0, 0);
let origen = Punto(0, 0, 0);
```

Aunque esto es posible, casi siempre es mejor usar una *struct* que una *tuple struct*. El ejemplo anterior podria quedar así:

```
struct Color {
    rojo: i32,
    azul: i32,
    verde: i32,
}

struct Punto {
    x: i32,
    y: i32,
    z: i32,
}
```
Si ya hiciste lecciones de Scrypto veras que las estructuras (struct) son un elemento esencial de los blueprint, seguiremos hablando de ellas para que las aprendas y comprendas a la perfección.

::: warning Resumen
- Las estructuras nos permiten almacenar en un solo contenedor datos relacionados
:::

