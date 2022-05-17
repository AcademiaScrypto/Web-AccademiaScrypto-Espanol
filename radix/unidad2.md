# Radix Engine

En la lección [¿Qué es Radix?](/radix/unidad1.md) vimos la estrecha relación que existe entre Radix Engine y Cerberus, el protocolo de consenso super fragmentado de Radix. De hecho gracias a que conviven en perfecta armonía Radix Engine y Cerberus, las dApps creadas sobre la red Radix, podrán escalar sin límites (*Nota: Esto ocurrirá en 2023 con Xi'an*).  

![escalabilidad](./img/escalabilidad.png)

Radix Engine es la capa de aplicación de la red Radix. Es donde se encuentran las reglas que utilizan los nodos para determinar que se puede y que no se puede guardar en el libro de registros de la red.   

Los desarrolladores, es decir Tú, también pueden desarrollar sus propias reglas. Estas reglas estarán definidas dentro de los *Componentes* que a su vez se pueden ensamblar como si fueran piezas de lego para crear dApps de forma sencilla y segura. Y todo esto gracias al Radix Engine.

Actualmente en *Olympia* la red pública de radix implementa la version 1 de Radix Engine, pero a partir del 15 de diciembre de 2021 con Alexandria radix pondrá en marcha la segunda iteración del Radix Engine. 

En la actualidad Radix Engine trata a los tokens como si fueran objetos 'físicos' y los guarda en las cuentas que actúan como si fueran bóvedas (Vaults). Esto lo puede conseguir gracias a la utilización de un modelo de máquina de estados finitos (FSM) bien restringido.  

![fsm](./img/activo.png)

Sin duda Radix Engine v1 ya ofrece un modelo mucho más intuitivo y fácil de usar. Pero no queda aquí, la nueva iteración del Radix Engine agrega *computación* descentralizada.  

Radix Engine v2 incorporará los contratos inteligentes y con ellos el cálculo de la poderosa lógica. Además incluirá un novedoso lenguaje de programación orientado a activos llamado *Scrypto*, así como un sistema de mensajeria entre contratos inteligentes. Radix Engine v2 utilizará todos los activos como si fueran *recursos* de la red de primera clase, con lo que la complejidad de los contratos inteligentes disminuye drasticamente.

![v2](./img/radixv2.png)

Gracias a la aplicación del modelo de maquina de estados finitos en todos los recursos de la red Radix, Radix Engine v2 logra que los contratos inteligentes sean fáciles, seguros, reutilizables y componibles. 




::: warning Resumen:
- Radix Engine v2 incorporará: 
    - Contratos inteligentes
    - Scrypto: un nuevo lenguaje de programación orientado a activos
    - Extendera el FSM a todos los recursos + mensajeria entre contratos inteligentes
:::

### Contenido Extra
- [The Problem with Smart Contracts Today](https://www.radixdlt.com/post/the-problem-with-smart-contracts-today)
- [How Radix Engine is Designed to Scale dApps](https://www.radixdlt.com/post/how-radix-engine-is-designed-to-scale-dapps)

### Bibliografía
- [Radix Engine v2: An Asset-Oriented Smart Contract Environment](https://www.radixdlt.com/post/radix-engine-v2-an-asset-oriented-smart-contract-environment)
