# Ramas

![Git básico](/git_ramas.svg)

Para la creación de proyectos sencillos una buena metodología es tener una rama (branch) principal **master** y una rama **development** donde comprobaremos que todos los cambio funcionen correctamente antes de enviar a **master** y luego tendremos multiples "features branches" para que cada desarrollador pueda encargarse de una tarea definida, trabajarla y probarla a gusto antes de incluir esos cambios en la rama **development**. 

### Pasos para empezar a trabajar en una feature
Supongamos que queremos trabajar en una nueva **característica** (feature). Primero que todo debemos crear una branch exclusiva para trabajar en la feature. Esta branch debe crearse a partir de **development**, no a partir de **master**. Cuando abrimos *git* en la carpeta en la que clonamos este repositorio deberíamos ver en que rama estamos, para ello utilizaremos el siguiente comando:

```rust
$ git branch
* master (pude ser main u otro nombre similar)
```
Ahora, nosotros queremos partir nuestro trabajo desde **development**, por lo que debemos movernos a esta rama:

::: tip
- La bandera **-b** la usaremos solo si la rama a donde queremos movernos no existe, la creara y nos moverá. 
:::

```rust
$ git checkout -b development 
Switched to a new branch 'development'
```

Lo que esto nos dice es que nuestra carpeta local ha hecho cambios para lucir como la rama **development**. Si hay diferencias entre la rama **master** y la rama development podremos verlas, por ejemplo, si es que nos falta un archivo que está en master pero no en development. **Lo importante ahora es que nos encontramos en la rama en la que queremos partir**. El siguiente paso es crear la *"feature branch"* en la que trabajaremos, para lo que debemos hacer:

```rust
$ git checkout -b feature_branch development
Switched to a new branch 'feature_branch'
```

Lo que esto nos dice es que hemos creado localmente una rama "feature_branch", que es una sub-rama de **development**. Pero como queremos trabajar con el servidor remoto, en este caso Github, debemos notificarle que "conecte" la rama local "feature_branch" con la branch remota "feature_branch":

```rust
git push --set-upstream origin feature_branch
```

Ahora estamos listos. Cuando hagamos nuestros cambios en el código, relevantes para la "feature" en la que estamos trabajando, cada vez que enviemos nuestros cambios (que hagamos *git push*), estaremos enviando los cambios a la branch "feature_branch".

### Pasos para terminar de trabajar en una "feature" (característica)

Una vez que todos los cambios necesarios para agregar una "feature" (característica) al proyecto ya fueron hechos, están listos los tests y todos aprueban los cambios, tenemos que enviar los cambios a la rama development. Para pasar los cambios de una rama a otra haremos *code review* a través de las **Pull Requests** que Github implementa (todo esto se hace en la página de Github). Después de terminar el trabajo, solo cuando la feature ya está aplicada y testeada en development tenemos que eliminar la branch correspondiente a la feature.

::: tip
- Para eliminar una rama debemos usar el siguiente comando:
```
git branch -d nombre_rama
// Ojo que para poder eliminar una rama no debes estar en ella.
```
:::

Desde Github primero seleccionaremos desde el menu horizontal **Pull Request** y segundo **Compare & pull request**:

![rama1](/git_rama1.png)

Primero podemos **seleccionar las ramas a comparar**, y donde se incorporaran los cambios. Segundo podemos **comparar** el código de la rama con la nueva característica hacia la rama destino. Para terminar de verificar que todo es correcto. Y tercero, una vez todo es verificado y ok, creamos un **pull request**:

![rama1](/git_rama2.png)

::: tip
- Si te diste cuenta, podemos dejar comentarios para explicar los cambios que vamos a aplicar. 
:::

Unimos las ramas, incorporando las nuevas características al codigo matriz.

![rama1](/git_rama3.png)

Y finalmente borramos la rama con las nuevas características:

![rama1](/git_rama4.png)

::: tip
- Recuerda borrar localmente la rama, para que no la veas entre tus ramas.
```
 git branch -d feature_branch
```
:::

::: warning Resumen
- Si no sabemos en que branch estamos: ***git branch***
- Para cambiar de branch: ***git checkout branch_objetivo***
:::