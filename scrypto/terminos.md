# 🔤 Terminos básicos

- **Blueprints:** plano, se define la estructura de la dapp, contiene la lógica, no mantiene un estado ni una dirección.
- **Component:** instancia a un blueprint, ahora si existe una direccion y un estado.
- **Package:** es una colección de blueprints que se compilan y publican como una sola unidad. Tiene una direccion.
- **Function:** en Scripto son estaticas, no requieren estado, se pueden llamar desde un blueprint
- **Method:** Se llama desde los componentes y debe tener una referencia a si mismo, requiere estado.
- **Resources:** los activos, "assets", son el corazon de Scripto y de REv2. Tienen que estar asociados a una cantidad, no 
se puede copiar ni destruir por accidente. Los 'resources' siempre estan en un 'Bucket' o un 'Vault'.
- **Bucket:** Contenedor temporal o transitorio de los 'resources', se crea en una transaccion y se destruye al finalizar la misma.
- **Vault:** Contenedor persistente de 'resources' y se almacena dentro de un componente. Se puede quemar en un 'Bucket'.
- **Token:** Es un 'resources' con cualquier cantidad y granularidad (decimales)
- **Badge:** Es un 'resource' pero que siempre es de granularidad 1 es indivisible y no pudes tener cantidad 0. Se usara para autorizaciones. 

### Bibliografia:
- [Presetación de Scrypto por CTO Russell Harvey](https://www.youtube.com/watch?v=he9TunEXgcY)