# Interfaces y HashMaps

## Tabla de contenidos
* Introducción
* Propiedades opcionales
* Tipos de Funciones
* Usando HashMaps
* HashMap vs Interface

## Introducción
Una interfaz en **Sflynlang** es un tipo de map. En otras palabras, puedes configurar 
la estructura de un grupo de palabras claves para mantener todo organizado.
```
interface Sflyn {
  name: string;
  year: number;
}

const sflyn: Sflyn = {
  name: 'Sflynlang',
  year: 2020,
};

print(sflyn->toString());
// Output: { name: Sflynlang, year: 2020 }


```
## Propiedades opcionales
Las interfaces tienen propiedades opcionales que pueden ser omitidas en la declaración
```
interface Label {
  name: string;
  size: Option<number>;
}

cont label1: Label = {
  name: 'Label 1',
};

print(label1->size->toString());
// Output: null

const label2: Label = {
  name: 'Label 2',
  size: 2,
};

print(label2->size->toString());
// Output: 2
```
## Tipos de funciones
```
interface Object {
  setName: (name: string) => void;
}

const obj: Object = {
  setName: (name: string): void => {
    print(name);
  },
};

obj->setName('Sflylang');
// Output: Sflynlang

```
Lee [Funciones](https://github.com/sflynlang/compiler/wiki/Functions-and-Anonymous-Functions) para mas información.

## Usando HashMaps
Si no quieres declarar una `interface` para tener el control de una variable, puedes usar HashMaps. Un `HashMap` no 
tienen un tipo definido, se generará en base al valor de la variable.
```const obj = {
  setName: (name: string): void => {
    print(name);
  },
};

obj->setName('Sflynlang');
// Output: Sflynlang

```
## HashMap vs Interface
Un `HashMap` solo puede usarse sobre variables, por otro lado, un `Interface` puede usarse para definir la estructura
de una clase, variable o como un tipo de dato.
