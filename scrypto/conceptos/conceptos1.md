# Básico 1

::: tip Vídeo
- [Conceptos básicos 1 y 2](https://youtu.be/8GxgVdDLujk)
:::

Lo primero que tenemos que saber es que Scrypto es un lenguaje de programación orientado a **activos**. Los **ACTIVOS** en Scrypto son lo mas importante y por tanto son nativos en la propia red, no viven en un contrato inteligente, tienen entidad propia y viven en la propia red de Radix. Esto quiere decir que son un tipo de recurso que cualquiera con permisos puede hacer uso de ellos, diríamos que los activos en Radix son "físicos".  
![Activos en Scrypto](./img/activo.png)

Ya sabemos que Scrypto es un lenguaje de programación orientado a activos, que con el podemos programar contratos inteligentes (que en Scrypto llamaremos Componentes) y con ellos a su vez dApps para el mundo DeFi. Ahora adentrémonos en terminología más técnica que nos ayudará a entender cómo funciona todo:
- Los **Blueprints** son en realidad los planos que vamos a escribir con Scrypto. Estos definiran la estructura y la lógica de nuestro contrato inteligente. 
- Un **Componente** es una *instancia* a un Blueprint, es como si los BluePrints fueran los planos de una casa, si queremos construir otra casa no hemos de crear unos nuevos planos solo utilizar los mismos. Una vez instanciado un Blueprint se convierte en un componente y ahora adquiere vida dentro de la red de Radix incluyendo una dirección propia donde encontrarlo así como su estado, datos y recursos. En definitiva los Blueprints son plantillas a parti de la cual se pueden crear copias de los componentes, seguramente personalizadas con diversos parametros de entrada.  

![Ejemplo de instancia simple](./img/instantiation_single.png)

### Bibliografia:
- [https://www.radixdlt.com/post/scrypto-an-asset-oriented-smart-contract-language](https://www.radixdlt.com/post/scrypto-an-asset-oriented-smart-contract-language)

