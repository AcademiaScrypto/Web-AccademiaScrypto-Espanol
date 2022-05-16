# Variables
Las variables en Rust se definen mediante la palabra clave: *let* 
::: tip
**Las variables:** Son inmutables de forma predeterminada
:::
Pongamos un ejemplo. (OJO: *El siguiente código aún no se compilará.*)
```
fn main() {
    let x = 1;
    println!("El valor x es: {}", x);
    x = 0; 
    println!("El valor x es: {}", x);
}
```
Leamos línea a línea:
1. Declaramos la función principal 'main', de momento quédate con eso.
2. **Aquí definimos la variable con nombre 'x' y valor 1**, utilizamos la palabra clave *let* seguido del nombre que queramos asignar a la variable más el símbolo igual (=) y el valor que queramos dar. 
3. Pintamos, en el terminal, el valor de x
4. Esto da error (*x=0;*), recuerda **en Rust las variables son por defecto inmutables**, es decir no se pueden cambiar su valor inicial.
5. Esta linea no se ejecutaría, debido al error anterior. 

Para solucionar este problema, podríamos hacer lo siguiente:
```
fn main() {
    let mut x = 1;
    println!("The value of x is: {}", x);
    x = 0;
    println!("The value of x is: {}", x);
}
```
¿Ves la diferencia?... está en la línea 2.  
Sí, se ha incluido una nueva palabra clave: *mut* entre la palabra clave *let* y el nombre de la variable. 
Con 'mut' la variable adquiere la mutabilidad, ahora podemos cambiar su valor tal y como se hace en la línea 4.

::: warning Resumen
- Las variables son inmutables por defecto
- *let*: permite crear variables
- *mut*: dota de mutabilidad a la variable.
:::

