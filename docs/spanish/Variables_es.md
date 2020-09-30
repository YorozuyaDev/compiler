# Variables

## Tabla de contenidos
* Introducción
* Tipos de datos
  * String
  * Numéricos
  * Booleanos
  * Null
  * Arrays

## Introducción
Declarar variables es facil, solo debes usar `let` o `const` seguido  del nombre y el valor, 
pero cual es la diferencia entre `let` y `const`?  

Cuando usas `let` para declarar variables, tu puedes asignarle un nuevo valor después de 
declararlo. En el otro caso, cuando usas `const`, esto no es posible.   

Si no especificas el tipo de dato de la variable, **Sflylang** tratará de inferirlo.

```
// String.
let name = "Sflyn";
const name1 = "Sflyn";

// Number.
let year = 2020;
const year1 = 2020;

// Boolean.
let isExample = true;
const isExample1 = true;

// Array.
let numbers = [0, 1];
const numbers1 = [0, 1];
```
O puedes especificar el tipo de dato:
```
let name: string = "Sflyn";
let year: number = 2020;
let isExample: boolean = true;
let numbers: number[] = [0, 1];
```
# Tipos de datos
**Sflylang** tiene cinco tipos de datos principales: `string`, `number`, `boolean`, `null` y arrays.
### String
Se utiliza para guardar texto. Puedes concatenar dos o más string (Si el valor no es de tipo 
string necesitas convertirlo con `toString()`).  
Puedes declarar strings usando comillas simples(') o comillas dobles(").
```
let name = "Sflyn";

name += 'lang';

print(name);
// Output: Sflynlang

print("Sflyn" + 'lang');
// Output: Sflynlang

```
### Numeros
Usado para guardar numeros reales y decimales. Con el tipo numérico puedes utilizar los siguientes operadores
aritméticos: plus (`+`), minus(`-`), multiply(`*`), exponenciación(`**` o `^`), módulo(`%`) y dividir (`\`).
```
let one = 1;
print(one->toString());
// Output: 1

let two = one + one;
print(two->toString());
// Output: 2

one -= two;
print(one->toString());
// Output: -1
```
### Booleanos
Los booleanos son `true` o `false`, este es el tipo de dato más básico en **Sflynlang**.
```
let isExample = true;
print(isExample->toString());
// Output: true

```
### Null
Null es un valor y un tipo de dato, puedes usarlo cuando tu no quieres asignar un string vacio o un 0.
```
let emptyString: Option<string> = null;
let emptyNumber: Option<number> = null;

// Then you can:
emptyString = "Sflynlang";
emptyNumber = 1;
```
### Arrays
**Sflynlang** solo soporta un tipo de dato por cada array.
```
let numbers = [0, 1, 2, 3];
print(numbers->toString());
// Output: [0, 1, 2, 3]

let letters = ['A', 'B', 'C', 'D'];
print(letters->toString());
// Output: [A, B, C, D]

```
Pudes obtener el elemento de un array de esta forma:
```
let numbers = [0, 1, 2, 3];

print(numbers[0]->toString());
// Output: 0

numbers[0] = 10;
print(numbers[0]->toString());
// Output: 10

print(numbers[10]->toString());
// Output: null

```
