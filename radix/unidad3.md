# Cerberus

Cerberus es el protocolo de consenso que sustenta Radix. Es el único protocolo que está diseñado para que todas las transacciones se compongan atomicamente en fragmentos. Esta es una característica fundamental para que las dApps puedan escalar a millones de usuarios.  

Actualmente Cerberus se ha implementado en Olympia de forma simplificada sin fragmentación. En esta versión  simplificada Cerberus es capaz de alcanzar al menos 50 transacciones por segundo que es más de 3 veces el rendimiento de Ethereum. Esta programado para 2023, con el lanzamiento de Xi'an, que el fragmentado de Cerberus permita escalar de forma lineal y prácticamente infinita preservando la composición atómica de las transacciones.

*Nota: tanto la fragmentación como la composición atómica, lo veremos en las dos siguientes unidades.*

Debajo de la [capa de aplicación](/radix/unidad2.md) se encuentra la capa de consenso, que define qué tan segura, escalable y descentralizada es la red.

Hasta el momento la capa de consenso Cerberus utilizara "un conjunto de validadores" que tendrá un tamaño fijo de 100 nodos, que seleccionará y actualizará continuamente entre aquellos nodos con mas XRD delegados. 

¿Pero qué es el consenso? es un problema de coordinación entre los nodos, el consenso debe lograr que los nodos estén de acuerdo o rechacen algo juntos. 

Un protocolo de consenso entonces es un conjunto establecido de procedimientos y reglas, que gobierna cómo se comunican entre sí y toman una decisión los nodos. 

![bft](./img/bft1.png)

Cerberus es un protocolo de consenso que además incorpora "estilo BFT" (Tolerante a fallos bizantinos) lo que lo hace un protocolo mas determinista. Y lo es porque en este tipo de consensos (BFT) desde un principio esta claro quien puede votar en una transacción y exactamente cuantos votos son necesarios para que todos puedan estar de acuerdo, no hay incertidumbre en todo el proceso. En este tipo de algoritmo de consensos se selecciona un *"lider"* que es el que propone la siguiente transacción para esa ronda. 

::: tip
Un epoch en Radix tiene 10.000 rondas.
:::

Los lideres dentro de Cerberus tienen como trabajo enviar mensajes a los nodos dentro de su conjunto de validadores y para impulsar el progreso a trave s de las *"fases"*, recoger los votos emitidos por los nodos para saber si estan de acuerdo y finalmente difundir el quorum, o no, entre los nodos. 

![fases](./img/consensus_phase.png)

Los pasos, "fases", para llegar al consenso son las siguientes:

- **Difusión**: El nodo *"lider"* emite un mensaje al conjunto de nodos validadores con una propuesta. 
- **Voto**: Todos lo nodos, validadores, votan si el mensaje es válido. Hacen esto por firma criptográfica y lo envian al lider.
- **Certificado de Quorum**: El lider reune los votos, si hay suficientes votos emite un certificado de quorum.

::: warning Resumen:
- Cerberus es la capa de consenso de Radix
- El consenso debe poner de acuerdo, o no, a los nodos.
- Un protocolo de consenso es un conjunto de reglas que gobiernan la comunicación y toma de decisión entre nodos.
:::

### Contenido Extra
- [Cerberus Whitepaper](https://assets.website-files.com/6053f7fca5bf627283b582c2/608811e3f5d21f235392fee1_Cerberus-Whitepaper-v1.01.pdf)
- [What is a Sybil attack?](https://learn.radixdlt.com/article/what-is-a-sybil-attack)
- [Are my funds safe on Radix?](https://learn.radixdlt.com/article/are-my-funds-safe-on-radix)

### Bibliografía
- [What is Cerberus?](https://learn.radixdlt.com/article/what-is-cerberus)
- [What is a consensus protocol?](https://learn.radixdlt.com/article/what-is-a-consensus-protocol)
- [How long is an epoch on Radix?](https://learn.radixdlt.com/article/how-long-is-an-epoch-on-radix)
- [Cerberus Infographic Series - Chapter V](https://www.radixdlt.com/post/cerberus-infographic-series-chapter-v)
