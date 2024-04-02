fn main() {
    // Esto es un comentario

    // Hola mundo
    println!("Hola, Rust!");

    // Variables
    let mut my_string = "Esto es un string";
    println!("{my_string}");

    my_string = "Aqui cambio el valor del string porque le he puesto mut delante";
    println!("{my_string}");

    // my_string = 6 => error

    let my_string2: String =
        String::from("Esto es otra cadena de texto String con un espacio fijo infereido con from");
    println!("{my_string2}");

    let mut my_int = 7;
    my_int = my_int + 4;
    println!("{my_int}");
    println!("Este es el valor de my_int: {}", my_int);

    let my_float = 5.43;
    // my_float = my_float + my_int; => esto da error porque no te deja sumar dos tipos distintos

    let my_bool = false;
    println!("{my_bool}");

    // Constantes
}
