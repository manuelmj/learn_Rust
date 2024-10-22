use std::collections::HashMap;



pub fn count_words(text: &str) -> HashMap<&str, u32> {
    let mut words_map: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count: &mut u32  = words_map.entry(word).or_insert(0); // devuelve una referencia al valor de la clave si ya existe, sino existe la crea e inserta 0
        *count += 1;
    }
    words_map
}
