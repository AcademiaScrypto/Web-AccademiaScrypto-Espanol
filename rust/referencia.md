# Prestamos y Referencias
En este tutorial de Rust, aprendemos cómo tomar prestado un valor temporalmente de su propietario, lo que nos permite tener múltiples referencias a ese valor sin romper la propiedad.

También aprendemos cómo hacer referencia a la dirección de memoria de un valor y cómo obtener el valor en esa dirección con el operador de desreferenciación.

### ¿Qué es esto?

Si queremos usar un valor en una función, sin transferir la propiedad a la función, podemos tomar prestado el valor temporalmente de su propietario. Cuando la función se realiza con el valor, se devuelve al propietario.

El préstamo nos permite tener una o más referencias a un mismo valor sin romper el concepto de “propietario único”.

Cuando tomamos prestado un valor, hacemos referencia a su dirección de memoria con el operador **&**. Una referencia en Rust es una dirección que se pasa a una función como argumento.

::: tip
- Si está familiarizado con el lenguaje de programación C, las **referencias** serían similares a los **punteros** .
:::

### Cómo hacer referencia y tomar prestado un valor de una variable
Usamos el operador **&** para hacer referencia a la dirección de memoria de un valor en Rust. Veamos cómo funciona esto con un ejemplo.

```rust
fn main() {

    let a = String::from("Hola");

    // referencia con &
    let str_len = get_str_length(&a);

    println!("La longitud del String es: {}", str_len);

}

fn get_str_length(s:&String) -> usize {

    s.len()
}
```

En el ejemplo anterior, la variable *a* es la propietaria de la cadena "Hola". Si usamos la variable en una función, pasará la propiedad a la función. En este caso no queremos eso, por lo que la solución es hacer referencia al propietario ( *a* ) en su lugar.

En la lista de parámetros de la definición de la función, agregamos el parámetro **&** antes del tipo para decirle a Rust que el valor entrante será una referencia a un valor, no el valor real. La función solo toma prestado el valor y lo devuelve después de que la función completa su ejecución.

Cuando llamamos a la función, tenemos que agregar el operador **&** nuevamente, esta vez delante del valor que le pasamos.

### Cómo cambiar un valor prestado
El valor prestado actúa casi como una variable regular, es inmutable. Pero, puede hacerse mutable con la palabra clave **mut**.

```rust
fn main() {

    let mut a = 5;

    cambiar_valor(&mut a);

    println!("{}", a);

}

fn cambiar_valor(num:&mut i32) {

    *num = 3;
}
```

Tenemos que agregar la palabra clave **mut** a la inicialización de la variable, la definición de la función (en la lista de parámetros) y la llamada a la función.

### Cómo desreferenciar una referencia (recuperar su valor)
Cuando usamos el operador de referencia ( & ), obtenemos la dirección de memoria del valor. Para obtener el valor real en la dirección, usamos el operador de desreferenciación ( * ).

```rust
fn main() {

    let mut a = 5;

    cambiar_valor(&mut a);

    println!("{}", a);

}

fn cambiar_valor(num:&mut i32) {

    // obtener el valor en la variable num
    // utilizando el operador *
    *num = 3;

}
```
En la definición de la función, usamos el operador de desreferenciación para obtener el valor en la dirección de memoria del parámetro y mutarlo.

::: warning Resumen
- Podemos tomar prestado un valor temporalmente de su propietario sin transferir la propiedad.
- Una referencia es la dirección de memoria de un valor y se pasa a una función como argumento.
- El operador de referencia es el símbolo **&**.
- Podemos encontrar el valor en la dirección de memoria con el operador de desreferenciación.
- El operador de desreferencia es el símbolo **(*)**.
- De forma predeterminada, un valor prestado es inmutable, pero podemos hacerlo mutable con la palabra clave **mut**.
:::
