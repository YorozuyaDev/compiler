# Funciones y Funciones Anónimas
## Tabla de contenidos
* Introducción
* Funciones Anonimas
* Valores por defecto en los argumentos
* Escribiendo funciones
## Introduccion
Las funciones se utilizan para ejecutar acciones cuando se invocan, para declararlas utilizamos
la palabra clave `func` seguida del nombre, los argumentos y el bloque a ejecutar.
El tipo de dato que devuelve por defecto es void y no devuelve ningún valor.
```
func printName() {
  print("Sflynlang");
}

printName();
// Output: Sflynlang

func getName(): string {
  return "Sflynlang";
}

print(getName());
// Output: Sflynlang
```
## Funciones anonimas
Una función anonima es similar a una función normal pero es declarada en una variable.
```
let printName = () => {
  print("Sflynlang");
};

printName();
// Output: Sflynlang

const getName = func (): string {
  return "Sflynlang";
};

print(getName());
// Output: Sflynlang
```
Además, puedes usar funciones anonimas como argumentos de funciones.
```
func getName(callback: (name: string) => void): void {
  callback('Sflynlang');
}

getName((name: string): void => {
  print(name);
  // Output: Sflynlang
});
```
## Valor por defecto en los Argumentos
Los argumentos de una función pueden tener valores por defecto. Si fijas un valor a uno,
los siguientes argumentos deben tener un valor por defecto también.
```
func concat(str: string, str2: string = 'lang'): string {
  return str + str2;
}

print(concat("Sflyn"));
// Output: Sflynlang

print(concat("Sflyn", "-lang"));
// Output: Sflyn-lang
```
## Escribiendo funciones
```
const addTwo: (n: number) => number = func (n: number): number {
  return num + 2;
}
```


