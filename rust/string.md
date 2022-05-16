# String

El tipo de datos String en Rust se puede clasificar en lo siguiente:

- Literal de cadena (&str)
- Objeto de cadena (String)

**Los literales** de cadena son un conjunto de caracteres, que están codificados en una variable. Por ejemplo:
```rust
 let web = "AcademiaScrypto.com";
 // Definimos el tipo
 let lugar: &str = "Toledo";
 ```

 El tipo de **objeto String** se puede utilizar para representar valores de cadena que se proporcionan en tiempo de ejecución. 

Para crear un objeto String, podemos usar cualquiera de las siguientes sintaxis:

```rust
// Crea una cadena vacía
let cadena_vacia = String::new();
// Crea una cadena con un valor por defecto
let cadena_defecto = String::from("AcademiaScrypto.com");
```

Los métodos más comunes son:

|Método         | Descripción |
|---------------|-------------|
|new()	        |Crea una nueva cadena vacía|
|to_string()    |Convierte el valor dado en una cadena|
|replace()	    |Reemplaza todas las coincidencias de un patrón con otra cadena |
|as_str()	    |Extrae un segmento de cadena que contiene la cadena completa|
|push()	        |Agrega el carácter dado al final de esta Cadena|
|push_str()	    |Agrega un segmento de cadena dado al final de esta Cadena|
|len()	        |Devuelve la longitud de este String, en bytes|
|trim()	        |Devuelve un segmento de cadena con los espacios en blanco iniciales y finales eliminados|
|split_whitespace()	    |Divide un segmento de cadena por espacios en blanco y devuelve un iterador|
|split()	    |Devuelve un iterador sobre las subcadenas de este segmento de cadena, separadas por caracteres que coinciden con un patrón|
|chars()	    |Devuelve un iterador sobre los caracteres de un segmento de cadena.|

Un valor de cadena se puede agregar a otra cadena. Esto se llama concatenación o interpolación. El resultado de la concatenación de cadenas es un nuevo objeto de cadena. Ejemplo:

```rust
let n1 = "academia".to_string();
let n2 = "Scrypto.com".to_string();

let n3 = n1 + &n2; // pasamos n2 como referencia y funciona. 
println!("{}",n3);
// resultado: academiaScrypto.com
```

Para convertir un número en una cadena utilizamos el método *to_string()* ejemplo:
```rust
let numero = 2022;
let numero_a_cadena = numero.to_string(); 

println!("{}",numero_a_cadena);
// resultado: "2022"
```

::: warning Atención
- Las cadenas de caracteres es uno de los temas mas farragosos de tratar en Rust, seguro que a lo largo de este curso dedicaremos mas de un momento a leer y probar documentación.
:::

### Contenido extra:

- [Almacenamiento de texto codificado en UTF-8 con Strings](https://saidgeek.github.io/rust-book/ch08-02-strings.html)