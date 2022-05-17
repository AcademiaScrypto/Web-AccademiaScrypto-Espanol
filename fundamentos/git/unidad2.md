# Básicos local

![Git básico](./img/git_basico.png)

### Workspace
📁 Es la carpeta local, es decir en nuestro computador, donde tenemos guardado nuestro proyecto. Para poder inicializar una proyecto con Git utilizaremos el comando *init*:

```
<carteta proyecto> git init
```
El comando *git init* creará una carpeta oculta *.git* y un archivo *.gitignore* en la carpeta/directorio local donde lo hemos ejecutado. *(Nota: Este paso solo lo realizaremos una vez)*

### Add
El comando *add* añade archivos al **Index** de Git. Con el *punto* después de *add* indicamos que queremos añadir todos los archivos que cuelguen del directorio actual incluidas sub-carpetas. 

```
git add .
```

### Index 
Este indice también es conocido como zona de montaje o área de preparación. En esta zona los archivos están en espera de ser guardados en repositorio. Para añadir archivos a esta área utilizaremos el comando *add* y para quitar archivos el comando *rm*.

```
git status
```
El comando *status* nos muestra el estado de los archivos en el *index*.

### Commit
La instrucción *commit* guarda todos los cambios realizados en los archivos que se encuentren en el *index de git* o área de montaje junto con una breve descripción del usuario que permitirá identificar rápidamente el conjunto de cambios guardados. La opción *-m* nos permite incluir un mensaje de no más de 50 caracteres. Estos mensajes de ven ser muy declarativos con el fin de ayudar en la comprensión del histórico de cambios. 

```
git commit -m "mensaje de nos mas de 50 caracteres"
```

### Repository

Es el lugar donde se guarda la colección de archivos con todas sus diferentes versiones. 

```
git log
```
El comando log nos muestra el listado de *commits* guardados en el Repository.

::: tip
- Hasta aquí sería el ciclo de vida con Git de forma local, básicamente:  
add -> commit
:::



