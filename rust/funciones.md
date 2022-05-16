# Funciones

Las funciones son una de las características más importantes de Rust.

Todo programa Rust posee al menos una función, la función **main**:
```
fn main() {   
}
```
Esta es la declaración de función mas simple posible. la palabra clave **fn** indica *‘esto es una función’*, seguido del nombre, paréntesis *(en este casos sin contenido ya que no recibe ningún argumento)* y luego llaves para indicar el cuerpo de la función.  

He aquí una función llamada ejemplo:
```
fn ejemplo() {   
}
```
Como ya hemos mencionado entre los **parentesis ()** de una función podemos incluir **argumentos**, estos argumentos se declaran de la siguiente manera:
```
fn imprimir_numero(x: i32) {
    println!("x es: {}", x);
}
```
En la funcion *imprimir_numero* (arriba indicada) incluimos como argumentos un *i32*, [recuerda de la lección de tipos de datos](/rust/tipodatos.md), que es un tipo de dato **número** *(en este caso con un tamaño de 32-bit)* el nombre que recibira este número dentro de la función sera *x*.  

La sintaxis es: nombre del argumento seguido de dos puntos y el tipo de dato. Ej: **casado: bool**, donde *casado* es el nombre del argumento y *bool* es el tipo de dato booleano.  

Si queremos pasar mas de un argumento en una funcion los debemos separar con comas: 
```
fn imprimir_suma(x: i32, y: i32) {
    println!("suma es: {}", x + y);
}
```
::: tip
- Aunque la inferencia de datos en Rust se puede reailzar dentro de las funciones, documentar tus tipos de manera explicita es una muy buena practica. 
:::

Ya hemos visto como pasar argumentos de entrada a una funcion, ahora veamos como retornar un valor:
```
fn suma_uno(x: i32) -> i32 {
    x + 1
}
```
La funciones en Rust retornan exactamente un valor, y el tipo es declarado después de una ‘flecha’, que es un guión (-) seguido por un signo mayor-que (>). La ultima linea de una función determina lo que esta retorna. En el caso anterior la funcion *suma_uno* devuelve un tipo de dato número entero *i32*.

::: tip
- Notaras la ausencia de un punto y coma en la última linea, esto es correcto para la última linea, que es la que en principio devuelve el valor. Esto es devido a que Rust es un lenguaje basado en experesiones: seguidamente veremos más acerca de ello.
:::

### Expresiones vs. Sentencias

Las expresiones retornan un valor, y las sentencias no, las sentencias deben terminar con un punto y coma.

*Nota: está es una explicación muy simplificada, ya daremos mas detalles!!!*

### Retornos tempranos

Rust posee una palabra clave para los retornos tempranos de una función, que es cuando la función retorna el valor sin ser la última línea de la función, la palabra clave es: **return**.  

En el siguiente ejemplo nunca llegaremos a la *expresión* de la última línea ya que terminará y retornara el valor de **x** con la *sentencia* **return**.
```
fn foo(x: i32) -> i32 {
    return x;

    // nunca llegaremos a este código!
    x + 1
}
```

### Funciones divergentes
😅 hablaremos en otro momento...

### Apuntadores a función
Vamos a realizar un acercamiento a los enlaces a variables que apunten a funciones, seguro que enseguida os dais cuenta de su potencial.

Declaramos una variable, tal y como vimos en la [unidad de variables](/rust/variables.md), y como tipo de dato ponemos una funcion de la siguiente manera:
```
let f: fn(i32) -> i32;
```
En este ejemplo a la variable le asignamos el nombre *f* y como tipo de dato una funcion que toma como argumento un numero entero *i32* y retorna un numero entero *i32*,

Veamos un ejemplo mas real, primero creamos una funcion muy sencilla:
```
fn mas_uno(i: i32) -> i32 {
    i + 1
}
```

Somos conscientes de que esta función toma como argumento un numero entero, le suma 1 y devuelve el resultado de dicha suma.  

Sigamos, creando una variable que apunte a esta función de la siguiente manera:
```
let f = mas_uno; // aquí estamos utilizando la inferencia de datos.
```

finalmente vamos a utilizar esta variable de la siguiente manera, observa como pasando 5, por ejemplo, el resultado debería ser 6, que es la sumas del argumento mas uno.

```
let seis = f(5);
```

::: warning Resumen
- Nuevamente, las funciones son el elemento más importante de Rust.
- fn seguido de parentesis *fn()* es la forma más basica de declarar una función
- podemos declarar tipos de valores como argumentos de entrada entre los parentesis de la funcion
- los tipos de valores de retorno se pueden declarar despues de los parentesis con una flecha **->**
:::

