# Match

Rust tiene un operador de flujo de control extremadamente poderoso llamado **match** que le permite comparar un valor con una serie de patrones y luego ejecutar el código según el patrón que coincida. 

::: tip
- Piense en una expresión de **match** como una máquina clasificadora de monedas: las monedas se deslizan por una pista con orificios de diferentes tamaños a lo largo de ella, y cada moneda cae por el primer orificio que encuentra que encaja. De la misma manera, los valores pasan por cada patrón en una coincidencia match, y en el primer patrón en el que el valor encaje, el valor cae en el bloque de código asociado para ser utilizado durante la ejecución.
:::

Podemos escribir una función que puede tomar una moneda y, de manera similar a la máquina de conteo, determinar qué moneda es y devolverá su valor en centimos, como se muestra aquí:

![billetes euro](/euros.png)

```rust
enum ColorBillete {
    gris,
    rojo,
    azul,
    naranja,
}

fn valor_por_color(color: ColorBillete) -> u32 {
    match color {
        ColorBillete::gris => 1,
        ColorBillete::rojo => 5,
        ColorBillete::azul => 10,
        ColorBillete::naranja => 25,
        _ => println!("Del resto no tenemos")
    }
}
```
La palabra clave **match** va seguida de una expresión, que en este caso es el valor *color*. Esto parece muy similar a una expresión utilizada con *if*, pero hay una gran diferencia: con if, la expresión necesita devolver un valor *booleano*, pero aquí, puede ser de **cualquier tipo**. 

Un brazo/rama de *match* tiene dos partes: un patrón y algo de código. El primer brazo aquí tiene un patrón que es el valor *ColorBillete::gris* y luego el *=>* operador que separa el patrón y el código para ejecutar. El código en este caso es solo el valor 1. Cada brazo está separado del siguiente con una coma.

Rust también tiene un patrón que podemos usar cuando no queremos enumerar todos los valores posibles. Por ejemplo, un u8 puede tener valores válidos de 0 a 255. Si solo nos importan los valores 1, 3, 5 y 7, no queremos tener que enumerar 0, 2, 4, 6, 8, 9 hasta 255. Afortunadamente, no es necesario: podemos usar el patrón especial **_** en su lugar.

El patrón _ coincidirá con cualquier valor. Al ponerlo después de nuestros otros brazos, el _ coincidirá con todos los casos posibles que no se hayan especificado antes. El **()** es solo el valor unitario, por lo que no sucederá nada en el caso _. Como resultado, podemos decir que no queremos hacer nada para todos los valores posibles que no enumeramos antes del marcador de posición _.






