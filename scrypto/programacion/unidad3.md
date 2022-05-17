# Escribimos nuestro primer Componente

::: warning
- Esta unidad está actualizada para Scrypto version 0.4 o mayor -> [Instrucciones actualización](/scrypto/instalacion/actualizacion.md)
:::

::: tip 📺
- [Soon Video Demostración]()
:::

Las fases para resolver un problema con una computadora suelen ser los siguientes, en el mundo de la programación:

- Análisis del problema
- Diseño del algoritmo
- Codificación
- Compilación y ejecución
- Verificación
- Depuración
- Documentación
- Mantenimiento

(*Nota: no siempre se siguen estos pasos*)

### Análisis del problema
**Problema**: Estamos trabajando para una 'Aseguradora' que nos pide que implementemos la temperatura de cierta ciudad a la red de Radix, con el fin de que otros contratos inteligentes puedan tener el dato (on-chain) para ejecutar pólizas de seguros. Debemos poder modificar la temperatura con el fin de tener actualizado el dato.

**Análisis**: 
Si te diste cuenta, la 'Aseguradora' nos esta pidiendo que creemos un mecanismo para introducir datos externos a la red de Radix (Oráculo). Como sabrás las redes DLT no conocen lo que sucede fuera de ellas, y la manera para introducir datos externos a una red DLT es a través de *transacciones*. 
::: tip
- Las redes DLT son Deterministas, que es indispensable para que los nodos lleguen a un consenso. 
:::
**División del problema:**
- Debemos crear un componente que guarde un dato de temperatura, nosotros sabemos que ese dato debe ser un *decimal* ya que las temperaturas contienen decimales.
- Debemos crear una método que permita leer el dato.
- Debemos crear un método que permita cambiar el dato.

Con esto hemos convertido un problema en diferentes sub-problemas mas pequeños, recogiendo todas las necesidades actuales. 

::: tip Muy importante:
- Como programadores debemos ceñirnos a lo que se nos ha solicitado, no inventemos ni agreguemos nuevas funcionalidades. 
:::

### Diseño del algoritmo

En este caso vamos a crear 3 algoritmos, uno para cada uno de los tres procesos básicos que nos han pedido. Para ello hoy utilizaremos el *diagramas de flujo*, es algo muy básico pero útil cuando empezamos a programar, en este caso los diagramas serán muy sencillos pero en otros casos más adelante pueden sernos muy útiles para clarificar las ideas. (*Nota: En contenido extra al final de esta unidad tendrás mas información sobre diseño de algoritmos*)

![diagrama](./img/diagrama1.png)

Ya hablaremos más sobre **algoritmos** y su diseño, pero de momento quédate con que en el diagrama *Crear Componente* se lee de la siguiente manera: Inicio -> Introducimos temperatura -> Procesamos la función *new* -> Finalizamos

¿Sabrías leer los dos siguientes diagramas? ¡Prueba!  

### Codificación

Primero vamos a crear el *Package*, recuerda que lo haremos desde la carpeta 'radixdlt-scrypto' ¡eso ya sabes hacerlo!🤪:  

*(Nota: vamos a asignar el nombre 'tiempo' pero tu puedes optar por cualquier otro)* 
```
scrypto new-package tiempo
```
Seguidamente vamos a entrar en la carpeta que se crea con el nombre *tiempo* recuerda, lo hacemos con el comando *cd*
```
C:\Users\<tu_usuario>\radixdlt-scrypto> cd tiempo
```

Abrimos el archivo *lib.rs* que está dentro de la carpeta *src* con el Visual Studio Code, una vez abierto limpiamos el blueprint del código ejemplo para dejarlo de la siguiente manera:
```rust
use scrypto::prelude::*;

blueprint! {
    struct Hello {
    }
    impl Hello {
        pub fn instantiate_hello() -> ComponentAddress {
            Self {
            }
            .instantiate()
            .globalize()
        }

        pub fn free_token(&mut self) {
           
        }
    }
}
```
Como ves, hemos limpiado el blueprint del código que trae como ejemplo, esto es algo que de momento haremos siempre al empezar un nuevo *Package*.  

::: danger Recuerda
- Los Blueprint constan de dos partes principales: **struct** e **impl**
- En *struct* declararemos los recursos o datos que contendrá el componente
- En *impl* crearemos las funciones y métodos necesarios para ejecutar las acciones requeridas, que podrán modificar o no el estado del componente que se encuentra en la *estructura* (struct).
:::

Seguidamente vamos a cambiar el nombre del *blueprint*, quitaremos el **Hello** y lo cambiaremos por **Temperatura** tanto en la estructura como en la implementación:

```rust
blueprint! {
    struct Temperatura {
        .
        .
    }
    impl Temperatura {
        .
        .
    }
}
```
::: tip
- Ojo: que los nombres de las struct deben comenzar con Mayúsculas. De momento quédate con eso, ya te explicare más.
:::

Vamos a empezar a codificar el primer algoritmo **'Crear componente'**, recuerda:
- Debemos crear un componente que guarde un dato de temperatura, nosotros sabemos que ese dato ser un *decimal* ya que las temperaturas contienen decimales.

Esto quiere decir que tenemos que declarar un dato dentro del componente que guarde una temperatura (temp) que es un dato de tipo decimal:
```rust
struct Temperatura {
       temp: Decimal
    }
```

Ahora hemos de crear la función que instancia el componente, por convención vamos a llamar a esa función *instantiate_temperatura*, que suele ser la suma de *instanciate y el nombre del blueprint*.

```rust
pub fn instantiate_temperatura(temp_inicial: Decimal) -> ComponentAddress {
            Self {
               temp: temp_inicial,
            }
            .instantiate()
            .globalize()
        }
```

Si te percataste, la función *instantiate_temperatura* tiene un parámetro de entrada de tipo *decimal* (que guardamos dentro de la variable: temp_inicial) y uno de salida de tipo *ComponentAddress*. Seguidamente dentro de *Self* (el cual referencia a los recursos que hemos declarado en la *struct*) pasamos el dato que hemos dado por entrada en la función *instantiate_temperatura* a el recurso permanente que se guarda dentro del componente de tipo coincidente *decimal* (temp). Finalmente instanciamos el blueprint con la expresión **.instanciate() y .globalize()** *(Nota: recuerda que como devuelve un valor no termina con punto y coma)*.

::: tip
- **.globalize()**: asigna una dirección para que las personas puedan acceder al componente. Dicha dirección se devuelve después a través del tipo *ComponentAddress*.
:::

Con esto habríamos codificado el primer algoritmo, en este caso una función 'constructora' que por convencionalismos llamaremos en este caso *instantiate_temperatura* y que es con la que daremos *vida* a nuestro *blueprint*. Una vez dicho esto, vayamos con el segundo algoritmo: **'Leer el Dato'**.

Para ello vamos a crear un método, recuerda que básicamente un **método** es una *función* que está ligada a una estructura de datos (struct) y hace referencia a esta a través de **self**. (*Nota: más adelante hablaremos de la habilidad que en Rust se denominada ‘sintaxis de llamada a métodos’*)

Estos métodos van a permitirnos ejecutar acciones dentro del *Componente*. Hemos dicho que un método es una función pues que vamos a escribir dentro del cuerpo de *impl*:

```rust
pub fn leer_temp(&self) {
}
```

Ya sabes que una función se declara con la partícula *fn* en este caso esta precedida de la partícula *pub* que hace referencia a que es una método *público*. Dentro de los paréntesis del nombre de la función, en este caso *leer_temp*, estamos pasando una referencia (&) a si mismo (self), y con esto ya tendríamos un método codificado.

¿Que dice nuestro algoritmo que debe hacer esta método? Devolver el mensaje: "La temperatura es: {temperatura} grados." vamos a codificarlo:

```rust
pub fn leer_temp(&self) {
    info!("La temperatura es: {} grados.", self.temp);
}
```
Ya te habrás percatado de que dentro de los {} corchetes se pintará por pantalla, junto al mensaje, el dato que se encuentra alojado en la estructura de nuestro Componente con el nombre *temp*. 

Nuestro código va quedando de la la siguiente manera, solo para que no pierdas la perspectiva:

```rust
blueprint! {
    struct Temperatura {
        temp: Decimal
    }
    impl Temperatura {
        pub fn instantiate_temperatura(temp_inicial: Decimal) -> ComponentAddress {
            Self {
               temp: temp_inicial,
            }
            .instantiate()
            .globalize()
        }

        pub fn leer_temp(&self) {
            info!("La temperatura es: {} grados.", self.temp);
        }
    }
}
```

Sigamos... ahora codificando el tercer algoritmo: **'Cambiar Temperatura'**, para ello nuevamente vamos a crear un método, ¡ya sabemos hacerlo 😎!, pero en este caso la referencia (&) a si mismo (self) tiene que ser mutable (mut), es decir que permita modificarse, para poder cambiar el dato que guarda el *Componente* dentro de la estructura. Quedaría de la siguiente manera:

```rust
pub fn cambiar_temp(&mut self)  {
}
```
Este tercer algoritmo nos pide que podamos modificar la temperatura, para ello hemos de permitir introducir un nuevo dato, esto lo logramos pasando un nuevo argumento a traves del método, recuerda que el dato es una temperatura que hemos declarado como tipo *Decimal* y por tanto, para que haya una concordancia de tipos, el dato que hemos de entrar debería ser también *Decimal*, quedaría algo así:

```rust
pub fn cambiar_temp(&mut self, nueva_temp: Decimal)  {
}
```
Finalmente guardamos el dato en la *estructura* dentro de la variable *temp* de la siguiente manera:

```rust
pub fn cambiar_temp(&mut self, nueva_temp: Decimal)  {
    self.temp = nueva_temp;
}
```

El operador igual (=) permite asignar valor y recuerda qwe esto lo podemos lograr gracias a que hemos declarado como mutable (mut) la referencia (&) a la estructura (self).

Con esto hemos codificado los tres algoritmos que nos pedían en las especificaciones iniciales del problema, quedando el código de la siguiente manera y terminando esta fase:

```rust
use scrypto::prelude::*;

blueprint! {
    struct Temperatura {
        temp: Decimal
    }
    impl Temperatura {
        pub fn instantiate_temperatura(temp_inicial: Decimal) -> ComponentAddress {
            Self {
               temp: temp_inicial,
            }
            .instantiate()
            .globalize()
        }

        pub fn leer_temp(&self) {
            info!("La temperatura es: {} grados.", self.temp);
        }

        pub fn cambiar_temp(&mut self, nueva_temp: Decimal)  {
            self.temp = nueva_temp;
        }
    }
}
```

### Compilación y ejecución

Una vez superada la fase de *Codificación* pasamos a compilar y ejecutar:

::: tip
- Antes de nada vamos a limpiar nuestro *simulador* para poder compilar y ejecutar sin rastros anteriores con el siguiente comando de resim:
```
resim reset
```
:::

Compilar es sencillo, en nuestro caso no solo compila sino que también integra en la red el *Package* asignándole una dirección que guaramos dentro de una variable se sistema:

```
resim publish .
```

Seguidamente guardamos la dirección del Package asignada (recuerda que en linux usamos *Export*):

```
set package <dirección del package>
```
::: tip
En Linux y MacOs utilizaremos *export key=value* ejemplo:
```
export package=<dirección del package>
```
:::

Creamos una nueva cuenta, que al ser la inicial quedara como la 'por defecto':

```
resim new-account
```
Guardamos nuestra dirección en una variable de sistema:
```
set acct <dirección de la nueva cuenta>
```

Ahora vamos a ejecutar la función 'constructora' *new* que **instancia** nuestro blueprint y lo convierte en un **Componente**, recuerda que hemos de pasa como argumento la temperatura inicial en este caso 20.5, y lo hacemos de la siguiente manera:
```rust 
resim call-function $package Temperatura instantiate_hello 20.5
```

Si todo ha salido bien... Ahora guardaremos en una variable de sistema la dirección del componente:

```
set component <dirección del componente>
```
::: tip
- Podemos ver el contenido del componente y verificar el dato de temperatura que hemos pasado como argumento
```
resim show $component
/* Resultado algo parecido a esto:
State: Struct { 20.5 }
*/
```
:::

Pasemos a comprobar el funcionamiento del método *leer_temp*:

```rust
resim call-method $component leer_temp
```
Si todo fue correcto, resultado ha de ser algo parecido a esto:

```
Logs: 1
└─ [INFO ] La temperatura es: 20.5 grados.
```
Ahora le toca el turno a el método *cambiar_temp*, en este caso además vamos a pasar la nueva temperatura:

Valor positivo:
```rust
resim call-method $component cambiar_temp 25
```

Valor negativo:
```rust
resim call-method $component cambiar_temp -- -5.2
// tenemos que poner delante del dato negativo dos guiones, 
// recuerda que los decimales se escriben con puntos y no con comas.
```


Si todo salio bien podemos comprobarlo volviendo a ejecutar por ejemplo el método *leer_dato*. ¡Ya sabes como hacerlo!

### Verificación y Depuración

En este caso con la propia ejecución ya hemos verificado la correcta ejecución del *Componente* y además hemos depurado, si hubiera sido necesario habríamos subsanando los errores producidos en la codificación. En algunos casos al codificar nos podemos percatar de que hayamos cometido errores en las fases de análisis y diseño, ahora estaríamos en el momento de volver a estas fases para corregir los errores y volver a codificar, compilar, ejecutar y verificar. 

![verificacion](./img/verificacion.png)

*Nota: Más adelante seguiremos explicando como en esta fase podemos implementar test (unitarios, funcionales, punta a punta, etc..) para verificar y depurar nuestros Blueprints.*

### Documentación y Mantenimiento

Finalmente quedaría la fase de Documentación donde deberíamos describir los pasos dados en el proceso de resolución del problema y ejecución del programa. En la programación profesional existen diversos documentos que hay entregar, en este caso vamos simplemente a comentar nuestro código. 

::: tip  En Rust para comentar:
- **//** Comentario de una sola línea
- /* ... */ Comentario multilinea 
- **///** Genera documentación de biblioteca para el siguiente elemento.
- **//!** Genera documentación de biblioteca para el elemento adjunto.
:::

Algo sencillo para nuestro blueprint seria así, los comentarios pueden ser algo subjetivos, normalmente los equipos de programación tienen unas normas y estilo claros, a la hora de codificar y documentar, para que todo el mundo se entienda:

```rust
use scrypto::prelude::*;

blueprint! {
    struct Temperatura {
        // Define un dato de tipo decimal para guardar la temperatura que luego utilizaremos en el componente.
        temp: Decimal
    }
    impl Temperatura {
        // Esta función despliega el componente, hay que pasar como argumento un decimal
        pub fn instantiate_temperatura(temp_inicial: Decimal) -> ComponentAddress  {
            // Instanciamos el componente Temperatura y poblamos el dato 'temp' con una temperatura inicial 
            Self {
               temp: temp_inicial,
            }
            .instantiate()
            .globalize()
        }
        // Este método devuelve un mensaje con la temperatura guardada en el dato 'temp'
        pub fn leer_temp(&self) {
            info!("La temperatura es: {} grados.", self.temp);
        }
        // Este método cambia el dato 'temp', se debe pasar un dato de tipo decimal
        pub fn cambiar_temp(&mut self, nueva_temp: Decimal)  {
            self.temp = nueva_temp;
        }
    }
}
```
El mantenimiento consta de: actualización de los cambios/mejoras planteados y corrección de posibles nuevos errores.

Enhorabuena... ya has podido escribir y ejecutar tu primer componente. Siéntete muy muy muy orgulloso.  En las próximas lecciones ampliaremos el ejemplo y eso sería parte, en principio, parte del mantenimiento . Déjanos tus sugerencias!!!

::: warning Importante:
- Soy muy consciente de que hay muchas que no has entendido, ¡TRANQUILO!, no te rindas, las entenderás... 😉
:::

### Contenido extra
- [Diseño de algoritmos en la programación de computadoras](https://www.monografias.com/trabajos94/diseno-algoritmos-programacion-computadoras/diseno-algoritmos-programacion-computadoras.shtml)
- [Herramienta para dibujo de diagramas de flujos](https://lucid.app/)
- [Metodología para la Construcción de Programas, slide sencillas](https://slideplayer.es/slide/2269458/)