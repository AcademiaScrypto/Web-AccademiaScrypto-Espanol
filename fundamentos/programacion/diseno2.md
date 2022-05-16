# Proceso Desarrollo - Diseño 2

![diseño](/diseño.png)

### Problemas complejos

La soluciones a problemas complejos pueden requerir muchos pasos. Las estrategias seguidas usualmente a la hora de encontrar algoritmos para problemas complejos son:

- **Partición o divide y vencerás:** consiste en dividir un problema grande en unidades más pequeñas que puedan ser resueltas individualmente.
    - Ejemplo: Podemos dividir el problema de limpiar una casa en labores más simple correspondientes a limpiar cada habitación.
- **Resolución por analogía:** Dado un problema, se trata de recordar algún problema similar que ya esté resuelto. Los dos problemas análogos pueden incluso pertenecer a áreas de conocimiento totalmente distintas.
    - Ejemplo: El cálculo de la media de las temperaturas de las provincias castellano manchegas y la media de las notas de los alumnos de una clase se realiza del mismo modo.

La descomposición del problema original en sub-problemas más simples y a continuación dividir estos sub-problemas en otros mas simples se denomina diseño descendente (top-down design). Tras la primera descripción del problema (poco específica), se realiza una siguiente descripción mas detallada con mas pasos concretos. Este proceso se denomina refinamiento del algoritmo.

**Ejemplo de diseño:**

Leer el radio de un circunferencia y calcular e imprimir su superficie y su circunferencia.

- Se puede dividir en tres sub-problemas más sencillos:
    - Leer Radio
    - Calcular Superficie
    - Calcular Longitud
    - Escribir resultados  

- Refinamiento del algoritmo:
    - Leer Radio
    - Superficie <- PI * Radio ^ 2
    - Longitud <- 2 * PI * Radio
    - Escribir Radio, Longitud, Superficie
Lo podemos ver en un diagrama estructurado:

![topdown](/algoritmo_estructurado.png)

### Contenido Extra:
- [Curso de introducción a la programación con pseudocódigo](https://plataforma.josedomingo.org/pledin/cursos/programacion/)
- [DISEÑO ESTRUCTURADO DE ALGORITMOS](http://www.geocities.ws/apuntesitpn/dise/temario.htm)
