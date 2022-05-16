# Proceso Desarrollo - Análisis

![Análisis](/analisis.png)

### Modelo en cascada
Aunque existen diferentes modelos para definir el ciclo de vida de un software, en este curso vamos ha hablar del modelo *Cascada* que es el más sencillo con el que empezar a programar. En este modelo las fases anteriores funcionarán una detrás de la otra de manera lineal. De este modo, solo cuando una fase termine se podrá continuar con la siguiente, y así progresivamente.

::: tip
- **"Pensar antes de programar" 😉**
:::

### Análisis

La fase de análisis define los requisitos del software que hay que desarrollar. Comenzaremos con una entrevista individual o una dinámica de grupo con los clientes. En ella también tendremos que saber lo que el cliente quiere o lo que cree que necesita.

Es importante que se mantenga una comunicación bilateral, y es necesario un consenso por ambas partes para llegar a definir los requisitos del software. Para ello se crea un informe ERS (Especificación de Requisitos del Sistema).

En esta fase, los requisitos que se deben definir son:

*Requisitos funcionales:* Con estos requisitos describiremos detalladamente lo que realiza el sistema y como reacciona antes diferentes entradas y situaciones.  

*Requisitos no funcionales:* Con estos requisitos detallaremos por ejemplo la capacidad de almacenamiento o la fiabilidad del sistema. Estos requisitos no incluyen como funciona el sistema.


Para poder representar estos requisitos, disponemos de varias herramientas. Diagramas de flujo, de transición de estados, diagrama de clases, de Entidad/Relación o diccionario de datos.

![analisis_1](/analisis_problema.png)

### Ejemplo de análisis

Leer el radio de un circunferencia y calcular e imprimir su superficie y su longitud.

**Análisis**

**Definición del problema:** Tenemos que saber que es el radio de un circunferencia, y saber que es su área y su longitud. Además tenemos que saber cómo calcular el área y la longitud. Por lo tanto necesitamos saber el radio y utilizar las formulas para calcular el área y la longitud.

|Especificaciones||
|----------------|-----------------------------------------|
|Entradas|	Radio de la circunferencia (Variable RADIO).|
|Salidas|	Superficie de la circunferencia (Variable SUPERFICIE).  Longitud de la circunferencia (Variable LONGITUD)|
|Variables|	RADIO, SUPERFICIE, LONGITUD de tipo REAL.|

Los datos de entrada y la información de salida se van a guardar en variables, donde se puede guardar datos. Las variables son de distintos tipos de datos: entero, real, cadena, booleano,..


### Contenido Extra:
- [Técnicas para Identificar Requisitos Funcionales y No Funcionales](https://sites.google.com/site/metodologiareq/capitulo-ii/tecnicas-para-identificar-requisitos-funcionales-y-no-funcionales)
- [Especificación de requerimientos - Universidad de Granada](https://elvex.ugr.es/idbis/db/docs/design/2-requirements.pdf)
- [Video: Requerimientos Funcionales y No Funcionales en desarrollo de software](https://www.youtube.com/watch?v=SIr2qP59dA0)
- [Ejemplo muy completo - Documentos requerimientos](https://www.enabel.be/sites/default/files/tenders/anexo_a_requerimientos_funcionales_y_no_funcionales.pdf)
- [Ejemplo simple - Documento requerimientos](http://www.lsi.us.es/~javierj/cursos_ficheros/02.%20Un%20ejemplo%20de%20requisitos.pdf)
- [Video: GUÍA 3 ESPECIFICAR LOS REQUISITOS FUNCIONALES Y NO FUNCIONALES DEL SISTEMA](https://youtu.be/6oh4QXoBpUY)