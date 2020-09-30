# Sflynlang Compiler
Este repositorio contiene el código del compilador escrito en Rustlang par Sflylang

## Qué es Sflynlang?
Sflylang es un lenguaje de programación multiparadigma y multiplataforma. El princpal objetivo es tener un lenguaje similar a TypeScript pero nativo para el explorador u otras aplicaciones; la sintaxis pretende ser mas sencilla y familiar para todo el mundo.

## Pre-requisitos
* Rustlang
* Rustup
* Rustlang VS Code Extension (Optional)

## Instalación
1. Ve a releases y clicka en la primera release que encuentres.
2. Descarga el binario del compilador para tu sistema operativo.
* Linux: `sflyn-x86_64-unknown-linux-gnu.zip`
* Windows: `sflyn-x86_64-pc-windows-msvc.zip`
* MacOs: `sflyn-x86_64-apple-darwin.zip`
3. Descarga la libreria STD y el resto de depedencias (archivo `sflyn-src.zip`)
4. Extrae `sflyn-src.zip` en `C:/sflynlang`, `$HOME/sflynlang`, o donde quieras.
5. Añade el path del STD a la variable del entorno `SFLYN_PATH`.
> Ejemplo: export SFLYN_PATH=$HOME/sflynlang.
6. Añade el binario del compilador sflylang a `SFLYN_PATH/bin/`.
7. Añade al path el path del enotorno.
> Ejemplo: export PATH=$PATH:$HOME/sflynlang/bin.
8. Ahora ya puedes correr código Sflyn!. Echa un vistazo a [Comenzando](#Comenzando).

## Como contribuir
Por favor lee la guia de contribuciones y el código de conducta.

## Lista de cambios
Los ultimos cambios en CHANGELOG.md

## Comenzando
Para empezar a programar en Sflyn, puedes compilar tu primer `hola mundo!`:
1. Crea un nuevo archivo que se llame `index.sf` con la siguiente linea:
```
print(`Hello World!`);
```
2. Para correr el programa usa el siguiente comando:
```
$ sflyn /path/to/index.sf
# Output: Hello world!
```
3. Genial! Has creado tu primer código en Sflyn.
## Wiki
---
Puedes encontrar mas sobre como trabajar con Sflynlang y su sintaxis en nuestra wiki.
## Ejemplos
---
* [Hello World](https://github.com/sflynlang/compiler/blob/main/examples/hello_world.sf)
* Clases
  * [Clases Sflyn](https://github.com/sflynlang/compiler/blob/main/examples/classes/Sflyn.sf)
* For 
  * [For en un array](https://github.com/sflynlang/compiler/blob/main/examples/for/array.sf)
  * [For en un hashmap](https://github.com/sflynlang/compiler/blob/main/examples/for/hashmap.sf) 
* Funciones
  * [Callback](https://github.com/sflynlang/compiler/blob/main/examples/functions/callback.sf)
  * [Double Number](https://github.com/sflynlang/compiler/blob/main/examples/functions/double.sf)
  * [Menos o mayor](https://github.com/sflynlang/compiler/blob/main/examples/functions/less_or_greater.sf)
  * [Di hola](https://github.com/sflynlang/compiler/blob/main/examples/functions/say_hi.sf)
* Interfaces
  * [Label](https://github.com/sflynlang/compiler/blob/main/examples/interfaces/label.sf)
* Modulos
  * [Basico](https://github.com/sflynlang/compiler/tree/main/examples/modules/basic)
* Variables
  * [Array](https://github.com/sflynlang/compiler/blob/main/examples/variables/arrays.sf)
  * [Booleanos](https://github.com/sflynlang/compiler/blob/main/examples/variables/booleans.sf)
  * [Constantes](https://github.com/sflynlang/compiler/blob/main/examples/variables/const.sf)
  * [Numeros](https://github.com/sflynlang/compiler/blob/main/examples/variables/numbers.sf)
  * [Strings](https://github.com/sflynlang/compiler/blob/main/examples/variables/strings.sf)
  
## Redes Sociales
* [Server de Discord](https://discord.com/invite/XdeRFHt)
* [Twitter(@sflynlang)](https://twitter.com/sflynlang)
* [Facebook (@sflynlang)](https://www.facebook.com/sflynlang)

## Contribuidores
* Daniel Solarte - Trabajo Inicial - [Github](https://github.com/danielsolartech)
* Maria Antonella - Icon Desing - [Instagram](https://www.instagram.com/raccon_324/)
* LemonCod3 - Apoyo emocional -  [Organización Github](https://github.com/LemonCod3) 
Puedes ver una lista de [contribuidores](https://github.com/sflynlang/compiler/graphs/contributors) aqui.

## Licencia
Este proyecto es desarrollado bajo licencia MIT. Lee [LICENSE](https://github.com/sflynlang/compiler/blob/main/LICENSE) para más información.

  
