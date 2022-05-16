# Capa de Transacciones

Radix, con la versión 0.3 del simulado de Scrypto, presenta su novedosa **Capa de Transacciones Orientada a Activos**. Será la encargada de orquestar los movimientos de recursos entre los componentes. En si misma es una transacción que está compuesta por una serie de llamadas a componentes donde podremos pasar datos o recursos y a su vez devolver recursos para su uso en esa misma transacción. 

::: tip
- Solo si toda la secuencia de instrucciones de una transacción es válida se procesará, si solo una de esas instrucciones falla toda la transacción fallará. 
:::

Con este tipo de transacciones complejas vemos de primera mano la fuerza de Radix Engine 2 + Scypto donde la componibilidad atómica es verdaderamente posible. 

Con esta forma de concebir las transacciones podremos realizar composiciones, llamando a diferentes componentes, conociendo y definiendo en todo momento el comportamiento y movimiento de los recursos. 

Ejemplo:

![transaccion](/transaccion1.png)


