# Vectores

Vector es una colección que puede aumentar de tamaño, solo puede contener un tipo de dato. Esta es la definición que ya vimos en la lección de [Tipos de Datos](/rust/tipodatos.md). Pero vamos a ir un poco más allá:

Los vectores le permiten almacenar más de un valor en una sola estructura de datos que coloca todos los valores uno al lado del otro en la memoria. Los vectores solo pueden almacenar valores del mismo tipo. Son útiles cuando tienes una lista de elementos, como las líneas de texto en un archivo o los precios de los artículos en un carrito de compras.

### ¿Como crear un Vector?
Para crear un nuevo vector vacío, podemos llamar a la función **Vec::new**:

```
let v: Vec<i32> = Vec::new();
```
En este caso seran valores de tipo *i32*

Los vectores se implementan usando genéricos: T; Por ahora, sepa que el tipo Vec T proporcionado por la biblioteca estándar puede contener cualquier tipo, y cuando un vector específico tiene un tipo específico, el tipo se especifica dentro de los corchetes angulares. En el ejemplo anterior, le hemos dicho a Rust que Vec T en v contendrá elementos del tipo i32.

En un código más realista, Rust a menudo puede inferir el tipo de valor que desea almacenar una vez que inserta los valores, por lo que rara vez necesita hacer esta anotación de tipo. Es más común crear un Vec T que tiene valores iniciales, y Rust proporciona la macro vec! Para mayor comodidad. La macro creará un nuevo vector que contiene los valores que le das. 

```
let v = vec![1, 2, 3];
```

Como hemos dado valores iniciales de i32, Rust puede inferir que el tipo de v es Vec i32, y la anotación de tipo no es necesaria. 

### Actualizando un Vector

Para crear un vector y luego agregarle elementos, podemos usar el método *push*:

```
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### Lectura de elementos de vectores

Hay dos formas de referenciar un valor almacenado en un vector. 

```
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
let third: Option<&i32> = v.get(2);
```

Tengamos en cuenta dos detalles aquí. Primero, usamos el valor de índice de 2 para obtener el tercer elemento: los vectores están indexados por número, comenzando por cero. Segundo, las dos formas de obtener el tercer elemento son usando *&* y *[]*, que nos da una referencia, o usando el método *get* con el índice pasado como argumento, lo que nos da una Opción <&T>

Rust tiene dos formas de referenciar el dato de un vector para que nosotros elijamos como se comporta el programa cuando intenta usar un valor de índice para el que el vector no tiene un elemento: Con *get* si no existe devuelve *none* y con *&* y *[]* si no existe el indice el programa entra en *pánico*.

### Iterando sobre los valores en un vector

Si queremos acceder a cada elemento en un vector a su vez, podemos iterar a través de todos los elementos en lugar de usar índices para acceder uno a la vez.

```
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

::: tip
Para cambiar el valor al que se refiere la referencia mutable, debemos usar el operador de desreferencia (*) para obtener el valor en i antes de poder usar el operador +=.
:::

### Usando un Enum para almacenar múltiples tipos

Al comienzo de este capítulo, dijimos que los vectores solo pueden almacenar valores que son del mismo tipo. Esto puede ser un inconveniente; definitivamente hay casos de uso para la necesidad de almacenar una lista de artículos de diferentes tipos. Afortunadamente, las variantes de una enumeración se definen bajo el mismo tipo de enumeración, de modo que cuando necesitamos almacenar elementos de un tipo diferente en un vector, podemos definir y usar una enumeración.

Por ejemplo, supongamos que queremos obtener valores de una fila en una hoja de cálculo en la que algunas de las columnas de la fila contienen números enteros, algunos números de coma flotante y algunas cadenas. Podemos definir una enumeración cuyas variantes contendrán los diferentes tipos de valores, y luego todas las variantes enum se considerarán del mismo tipo: la de la enumeración. Entonces podemos crear un vector que contenga esa enumeración y así, en última instancia, tenga diferentes tipos. 

```
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

Cuando estás escribiendo un programa, si no conoces el conjunto exhaustivo de tipos que el programa obtendrá en tiempo de ejecución para almacenarlo en un vector, la técnica enum no funcionará. En cambio, puede usar un objeto trait.

Ahora que hemos discutido algunas de las formas más comunes de usar vectores, asegúrese de revisar la documentación de API para todos los muchos métodos útiles definidos en Vec T por la biblioteca estándar. Por ejemplo, además de *push*, un método *pop* elimina y devuelve el último elemento. 



::: warning Resumen
- Los vectores nos pueden ayudar a definir listas de elementos de los que no conozcamos el tamaño. Y como hemos visto al final, no tienen porque necesariamente elementos de un solo tipo. 
:::

