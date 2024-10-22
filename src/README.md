# Ejercicios 

## _EJECICIOS CON STRING_
-  [x]  Validación de contraseñas: Escribe una función que tome un String como entrada y determine si cumple con ciertos 
criterios de contraseña (por ejemplo, longitud mínima, presencia de mayúsculas, minúsculas y números). Puedes utilizar métodos como len(), chars(), contains() y otros para realizar la validación.  [Solucion](/src/string_exercises/password_validation.rs)

- [X] Conteo de palabras: Escribe una función que tome un String como entrada y devuelva un mapa hash (HashMap) donde 
las claves sean las palabras del texto y los valores sean la cantidad de veces que aparece cada palabra.
Puedes usar el método split_whitespace() para separar el texto en palabras y luego usar un HashMap para contar las 
ocurrencias. [Solucion](/src/string_exercises/count_words.rs)

- [X] Conversión de mayúsculas y minúsculas: Escribe una función que tome un String como entrada y devuelva una nueva 
String donde todas las letras estén en mayúsculas o minúsculas, según se especifique. Puedes utilizar métodos como to_uppercase() y to_lowercase() para realizar la conversión. [Solucion](/src/string_exercises/uppercase_lowercase.rs)

## _EJECICIOS CON VECTORES_

- [X] Calculadora de estadísticas: Escribe una función que tome un vector de números (Vec<i32>) como entrada y calcule la media, la mediana y la moda de los números. Puedes utilizar métodos como sort(), len(), get() y un  HashMap para calcular la moda. [Solucion](/src/vec_exercises/statistics.rs)

- [X] Eliminación de duplicados: Escribe una función que tome un vector como entrada y devuelva un nuevo vector con 
todos los elementos duplicados eliminados. Puedes utilizar un HashSet para realizar un seguimiento de los elementos 
únicos. [Solucion](/src/vec_exercises/duplicate_items.rs)

    _Nota: Esta sugerencia utiliza un HashSet, que no se menciona en las fuentes proporcionadas. Es una estructura de datos similar a un HashMap, pero solo almacena claves, no valores. Puedes encontrar más información sobre HashSet 
    en la documentación de la biblioteca estándar de Rust._

- [x] Rotación de un vector: Escribe una función que tome un vector y un número entero k como entrada y rote el vector\<k> posiciones a la izquierda o a la derecha, según se especifique. Puedes utilizar rebanadas (slices) y concatenación
 de vectores para realizar la rotación. [Solucion](/src/vec_exercises/rotate.rs)

