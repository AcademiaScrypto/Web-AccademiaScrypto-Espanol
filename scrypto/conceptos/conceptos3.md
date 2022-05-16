# Básico 3
### Los Recursos
Ya hemos visto que *Scrypto* es un lenguaje orientado a activos, todos los activos en Radix son consideraos recursos dentro de la propia red. Y estos recursos deben mantenerse dentro de un **Contenedor** en todo momento.   
En radix existen, en principio, dos tipos de contenedores de recursos: **Vault** y **Buckets** *(Nota: Utilizaremos los nombres en inglés para evitar confusiones)*  
Una cuentas en Radix, no es más que, un titular de recursos que a su vez se guardan en un *Vault*.

### Contenedores
**Vault** es un contenedor **permanente**, normalmente este tipo de contenedor lo veremos en la definición de la estructura (*Struct*).  

**Bucket** es un contenedor **temporal**, estos deben vaciarse o quemarse al final de una funcion o método. Por esta razón nunca veras un *Bucket* dentro de la declaración *Struct* en un *Blueprint*, sino más bien en las parte de impelementación (*impl*).

### Resim
¿Que es el comando 'resim'? de las siglas **R**adix **E**ngine **Sim**ulatores es un comando (resim.exe) que nos permite interactuar con el simulador de la red de Radix desde el terminal.  

Aquí teneis las *flags* y *subcomandos* que podemos utilizar con **resim** *(Nota: en las próximas semanas anilizaremos mas en profundidad la mayoría de estos subcomandos)*

|Flags:||
|-------------|-----------------|
|-h, --help|Ayuda|
|-V, --version| Información de la version|

|Sub Comandos:|Acción|Sintaxis|
|-------------|-----------------|-----------------|
|call-function|Calls a function|```resim call-function <package_address> <blueprint_name> <function> <args>```|
|call-method |Calls a method|```resim call-method <component_address> <method> <args>```|
|export-abi|Exports the ABI of a blueprint|```resim export-abi <package_address> <blueprint_name>```|
|help|Prints this message or the help of the given subcommand(s)|``` ```|
|mint|Mints resource|```resim mint <amount> <resource_address> <minter_resource_address>```|
|new-account|Creates an account|```resim new-account```|
|new-badge-fixed|Creates badge resource with fixed supply|```resim new-badge-fixed <amount>```|
|new-badge-mutable|Creates badge resource with mutable supply|```resim new-badge-mutable <minter_resource_address>```|
|new-token-fixed|Creates token resource with fixed supply|```resim new-token-fixed <amount>```|
|new-token-mutable|Creates token resource with mutable supply|```resim new-token-mutable <minter_resource_address>```|
|publish|Publishes a package|```resim publish <path_to_package_dir>```|
|reset|Resets the data directory|```resim reset```|
|run|Compile and run a transaction manifest|``` ```|
|set-current-epoch|Sets the current epoc|``` ```|
|set-default-account|Sets the default account|```resim set-default-account <account_component_address> <account_public_key>```|
|show|Displays the content behind an address|```resim show <address>```|
|show-configs|Displays configurations|``` ```|
|show-ledger|Displays ledger summary|```resim show-ledger```|
|transfer|Transfers resource to another account|```resim transfer <amount> <resource_address> <recipient_component_address>```|

::: warning Resumen
- **Vault** es un contenedor *permanente* de recursos
- **Buket** es un contenerdor *temporal* de recursos
- **Resim** es el comando principal que nos permite interactuar con el Simulador.
:::

### Bibliografía:
[Command-line Tools](https://docs.radixdlt.com/main/scrypto/toolchain/commandline-tools.html)
