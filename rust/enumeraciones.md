# Enumeraciones

Primero decir que para muchos las *enumeraciones* y las *estructuras* son muy parecidas, para otros las enumeraciones son superiores y las estructuras son simplemente 'azúcar sintáctico'. Lo cierto es que Rust tiene una amplia gama de tipos de datos que da al programador cierta flexibilidad para definir los datos como mas le convenga.

- Estructuras con campos con nombre (struct Foo {bar: uint})
- Estructuras de tuplas (struct Foo(pub Bar, Baz))
- Estructuras sin campos (struct Foo;)
- Enums, con varios tipos de variantes:
    - Variantes sin campos (p. Ej. None)
    - Variantes de tupla (p. Ej. Some(T))
    - Variantes de estructura (por ejemplo, Some { pub inner :T })

Si te fijas las estructuras de campos con nombres y las enumeraciones con variante de estructura son muy muy parecidas. 

¿Pero que es una enumeración? El manual de Rust indica que es un *tipo de datos algebraico*, lo que quiere decir que es un tipo formado por la combinación de otros tipos. 

Cuando necesites modelar un dato que pueda tener distintos tipos o estados, los **enums** son la mejor herramienta con la que podemos proceder.

Para declarar un *enum* primero debemos escribir la palabra reservada **enum** seguido por un par de llaves. Dentro de las llaves debemos escribir todas las posibilidades que puede tomar el modelo que estamos tratando de definir. Estas son llamadas **variantes**. Esas variantes pueden ser definidas con o sin datos, los datos pueden ser tipos primitivos, estructuras (structs) o hasta otras enumeraciones (enums).

```rust
// Definimos un enum con las posibles direcciones
enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha
}

// Definimos un enum con las posibles acciones del jugador
enum JugadorAcción {
    Mover {
        direccion: Direccion,
        velocidad: u8,
    }, 
    Esperar, 
    Atacar(Direccion)
}
// Definimos las acciones con Match
match JugadorAccion {
    // Si la acción es esperar
    JugadorAccion::Esperar => println!("El jugador esta en espera"),
    // Si la acción es moverse obtenemos el la velocidad y dirección
    JugadorAccion::Mover { direccion, velocidad } => println!("El jugador se mueve {:?} con una velocidad de {}", direccion, velocidad),
    // Si la acción es atacar extraemos la dirección
    JugadorAccion::Atacar(direccion) => print!("El jugador ataca en dirección {:?}", direccion)
}

// Definimos una acción simulada
let accion_jugador = JugadorAccion::Mover {
    direccion: Directions::Abajo,
    velocidad: 2,
};

// El jugador se mueve Abajo con una velocidad de 2

```
Lo que estamos haciendo es usar la expresión **match** para mostrar un mensaje de acuerdo a la variante del enum que estamos recibiendo.

Veamos otro ejemplo:
```rust
enum Connection {
    Sucessfull,
    Failed, 
    TimedOut
}

impl Connection {
    fn get_status(&self) -> String {
        match self {
            Connection::Sucessfull => format!("200 OK"),
            Connection::Failed => format!("404 Not Found"),
            Connection::TimedOut => format!("Timed Out")
        }
    }
}

let connection_status = Connection::Sucessfull;
println!("Status: {}", connection_status.get_status())
// Status: 200 OK
```

En este caso para nuestro enum el método que definimos se asocia como un miembro para cada una de las variantes y si llamamos el método a través de alguna de las variantes de nuestro enum se ejecuta el método get_status el cual recibe la variante como argumento y se le puede aplicar la expresión match para enviar un resultado según la variante de donde se llame.


::: warning Resumen
- Una enumeración es un tipo de dato que pueda contener distintos tipos de datos.
:::

