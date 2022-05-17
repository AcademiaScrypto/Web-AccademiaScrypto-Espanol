![Banner Api](./img/6.png)

# 🔑 Radix API
Desde principios del año 2022, y tras una renovación completa, Radix nos ofrece al menos tres maneras diferentes de acceder a los datos de la red. Cada una de las formas responde a casos de uso diferentes y brindan la posibilidad a los desarrolladores acotar el acceso a datos a las necesidades de su proyecto. 
- Core API: está diseñada para el acceso de bajo nivel al estado del libro mayor, proporciona herramientas de construcción de transacciones y permite firmar mensajes con la clave privada del nodo, donde está habilitado.
- System API: está diseñada para la supervisión y el uso por parte de los corredores de nodos.
- Gateway API: esta puerta de enlace esta pensada para la creación de Wallets y Explorers, en principio es a la que le vamos a dar atención en la [Academia](/src/academia/README.md) ya que nos permite de forma sencilla y fácil acceder a los datos de las cuentas y transacciones realizadas en la red.

### Gateway API
El Gateway proporciona la API pública para Wallets y Explorers. Maneja las consultas de lectura utilizando la base de datos y mapea las solicitudes de construcción y envío a la API central de uno o más nodos completos. Radix ofrece un nodo Gateway para pequeñas pruebas pero recomienda que si realizamos un proyecto serio deberíamos correr nuestro propio nodo Gateway.

Una opción genial y que recomiendo es, centrarnos en el desarrollo de nuestra DApp, y dejar a otros el desarrollo y la implementación de la infraestructura. Para ello contamos con proyectos alucinantes como [Clana.io](https://clana.io/) quien pone a nuestra disposición una puerta de entrada a Radix escalable y altamente disponible. 

### ¿Qué es una API?
El término API es una abreviatura de *Application Programming Interfaces*, que en español significa interfaz de programación de aplicaciones. Se trata de un conjunto de definiciones y protocolos que se utiliza para desarrollar e integrar el software de las aplicaciones, permitiendo la comunicación entre dos aplicaciones de software a través de un conjunto de reglas.

Así pues, podemos hablar de una API como una especificación formal que establece cómo un módulo de un software se comunica o interactúa con otro para cumplir una o muchas funciones.

En nuestro caso nos permite interactuar con la red pública de Radix. Nos permite realizar consultas o enviar transacciones. 

*En este curso solo vamos a crear ejemplos con JavaScript, NodeJs y Firebase Functions.*

Os dejo un ejemplo de acceso a la API de pruebas de Radix, utilizamos el módulo *node-fetch* para NodeJs:

```javascript
import fetch from 'node-fetch';

var raw = JSON.stringify({"network_identifier": {"network": "mainnet"},
           "account_identifier": {"address": "rdx1qspqp0nyg3a3dj7e8vkd2ecxrgnrgvsl4d72efx2a4jwavfn995652c7f6uqy"},
           "cursor":"3",
           "limit": 1});


var requestOptions = {
  method: 'POST',
  headers: {
    "Content-Type": "application/json",
    "x-radixdlt-target-gw-api": "1.0.3",
  },
  body: raw,
  redirect: 'follow'
};

const response = await fetch('https://mainnet.radixdlt.com/account/transactions', requestOptions);
const data = await response.json();

console.log(data.transactions);
```


### Enlaces API
- [Documentación API RADIX](https://docs.radixdlt.com/main/apis/introduction.html)
- [Postman specification Gateway API](https://documenter.getpostman.com/view/14449947/UVXnHaJf)
