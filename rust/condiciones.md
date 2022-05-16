# Condiciones

Las condiciones en Rust se definen con las siguientes palabras clave: *if, else if, else*.   
*Nota: En Rust: no se necesita paréntesis alrededor de la expresión boolena, lo cual es excelente para la legibilidad.*
  
**if** es una forma especifica de un concepto mas general, el ‘branch’ (rama). El nombre proviene de una rama en un árbol: es un punto de decisión, en el cual dependiendo de una opción, multiples caminos pueden ser tomados.

Si solo utilizamos la particula **if** hay una sola eleccion que conduce a dos caminos:

```
let x = 5;

if x == 5 {
    println!("x es cinco!");
}
```

De haber cambiado el valor de *x* a algo diferente, esta linea (línea 4) no hubiese sido impresa. Para ser mas especifico, si la expresión después del *if* es evaluada a **true**, entonces el bloque de código es ejecutado. Si es **false**, dicho bloque no se invoca.

::: tip
- **==** es el operador de igualdad por comparación
- **!=** es el operador de no igualdad por comparación
- **>** es el operado de mayor que
- **<** es el operado de menor que
- [Más sobre operadores](https://doc.rust-lang.org/book/appendix-02-operators.html)
:::

Si queremos que *el arbol de decisión pueda tener más ramas* utilizaremos la particula **else** y **else if**:

```
let x = 5;

if x == 5 {
    println!("x es cinco!");
} else if x == 6 {
    println!("x es seis!");
} else {
    println!("x no es ni cinco ni seis :(");
}
```

Algo menos estándar seria lo siguiente:

```
let x = 5;

let y = if x == 5 { 10 } else { 15 }; // y: i32
```

Esto funciona porque if es una expresión. El valor de la expresión es el valor de la ultima expresión en el bloque que haya sido seleccionado.

::: warning Resumen
- Las condiciones nos permiten ejecutar diferentes opciones de código dependiendo del resultado de la expresión booleana observada en el bloque *if* o *else if*.
:::



