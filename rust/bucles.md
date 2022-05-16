# Bucles

Actualmente Rust provee tres manera de realizar actividades iterativas o bucles: **loop, while y for**. Cada uno de dichos enfoques tiene sus propios usos.

**Loop**: El ciclo infinito loop es el ciclo mas simple disponible en Rust. Rust proporciona una forma de iterar indefinidamente hasta que alguna sentencia de terminación sea alcanzada.
```
loop {
    println!("Itera por siempre!");
}
```
::: tip
 Se puede etiquetar el blucle loop con la siguiente sintaxis: *label: loop* y así poder romper con la particula **brake;** el bucle que deseemos. 
:::

**While**: normalemente *while* es la elección correcta de bucle si no estamos seguros de cuantas veces necesitamos iterar. Como con *if*, seguido de la particula *while* escribiremos una expresion a evaluar de manera booleana (true o false).

```
while !completado {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 {
        completado = true;
    }
}
```

::: tip
- *+=* (variable += expresión) es una sumas y asignaciones aritméticas
- *%* (expresión % expresión) devuelve el resto aritmetico.
:::

De necesitar un ciclo infinito, podrías sentirte tentado a escribir algo como esto:
```
while true {
}
```
Sin embargo, *loop* es por lejos, el mejor para este caso.

**For**: El blucle o ciclo *for* es usado para iterar un número particular de veces. *Nota: en Rust los for son algo diferentes a otros lenguajes*
```
for x in 0..10 {
    println!("{}", x); // x: i32
}
```
Si te percataste, en terminos abstractos la sintaxis de *for* seria así:
```
for var in expresion {
    código
}
```
En este caso la *expresión* (en el ejemplo: 0..10) es el iterador. El iterador devuelve una serie de elementos. Donde cada elemento es una iteración del bucle. Ese valor, fruto de la iteración, es a su vez asignado al el nombre de *var*., el cual se puede utilizar dentro del cuerpo del bucle. Una vez que el ciclo termina, el siguiente valor es obtenido del iterador, y se itera una vez más. Cuando no hay mas valores en el iterador, el ciclo *for* termina. 

::: tip
- 0..10 es una expresión que toma una posición de inicio y una de fin, y devuelve un iterador para esos valores. ¡Cuidado! no iterará con el 10 solo hasta el 9.
:::

Nota: En próximas lecciones de Rust sobre bucles o ciclos hablaremos de **Enumerate**, **Iteradores**, **break**, **continue** y **etiquetas para los loops**

::: warning Resumen
- **loop**: bucle infinito.
- **while**: no conocemos el numero de iteraciones.
- **for**: itera un número particular de veces.
:::

