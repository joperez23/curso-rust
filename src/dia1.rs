// Type Aliases
/* enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

// Aliases are more useful with long, complex types:
use std::cell::RefCell;
use std::sync::{Arc, RwLock};
type PlayerInventory = RwLock<Vec<Arc<RefCell<Item>>>>; */

// static y const
/* static BANNER: &str = "Welcome to RustOS 3.14";

fn main() {
    println!("{BANNER}");
}

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}
 */

// Enums
/* #[repr(i8)]
enum Bar {
    A = 6, // 0
    B,
    C, // 10001
}

fn main() {
    println!("A: {}", Bar::A as u32);
    println!("B: {}", Bar::B as u32);
    println!("C: {}", Bar::C as u32);
} */

// Enums 1
/* #[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Simple variant
    Run(Direction),              // Tuple variant
    Teleport { x: u32, y: u32 }, // Struct variant
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("On this turn: {:?}", m);
} */

// Estructuras de tuplas
/* struct Point(i32, i32);

fn main() {
    let p = Point(17, 23);
    println!("({}, {})", p.0, p.1);
}

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force);
} */

// Structs
/* struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old", person.name, person.age);
}

fn main() {
    let mut peter = Person {
        name: String::from("Peter"),
        age: 27,
    };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person {
        name: String::from("Jackie"),
        ..avery
    };
    describe(&jackie);
} */

// Geometría
/* /// Calculate the magnitude of the given vector.
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }
    mag_squared.sqrt()
}

/// Change the magnitude of the vector to 1.0 without changing its direction.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    vector[0] /= mag;
    vector[1] /= mag;
    vector[2] /= mag;
}

fn main() {
    println!(
        "Magnitude of a unit vector: {}",
        magnitude(&[0.0, 1.0, 0.0])
    );

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
} */

// Referencias colgantes
/* fn main() {
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
} */

// Referencias
/* fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);
    // *r = 'D'; --> Error
}

fn x_axis(x: i32) -> &(i32, i32) {
    let point = (x, 0);
    return &point;
} */

// Transponer una Matriz
/* fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], // <-- the comment makes rustfmt add a newline
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}*/

// Desestructurando Enums
/* #[rustfmt::skip]
fn main() {
    let triple = [0, -2, 3];
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z] => println!("First is 0, y = {y}, and z = {z}"),
        [1, ..]   => println!("First is 1 and the rest were ignored"),
        _         => println!("All elements were ignored"),
    }
}

fn main() {
    describe_point((1, 0));
}

fn describe_point(point: (i32, i32)) {
    match point {
        (0, _) => println!("on Y axis"),
        (_, 0) => println!("on X axis"),
        (x, _) if x < 0 => println!("left of Y axis"),
        (_, y) if y < 0 => println!("below X axis"),
        _ => println!("first quadrant"),
    }
} */

// Correspondencia de Patrones
/* #[rustfmt::skip]
fn main() {
    let input = 's';
    match input {
        'q'                       => println!("Quitting"),
        'a' | 's' | 'w' | 'd'     => println!("Moving around"),
        '0'..='9'                 => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                         => println!("Something else"),
    }
} */
/*
Puedes señalar cómo se usan algunos caracteres concretos en un patrón
    | como or,
    .. se puede ampliar tanto como sea necesario.
    1..=5 representa un rango inclusivo.
    _ es un comodín.
*/

// La declaración for admite la iteración sobre matrices (pero no sobre tuplas).
/* fn main() {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        for i in 2..prime {
            assert_ne!(prime % i, 0);
        }
    }
} */

// Tuplas y Arrays
/* fn main() {
    let mut a: [i8; 10] = [42; 10];
    a[5] = 0;
    println!("a: {a:#?}");
}

fn main0() {
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
} */

// Secuencia Collatz
/*
/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
} */

// Macros - println!(format, ..), format!(format, ..), dbg!(expression), todo!(), unreachable!()
/* fn factorial(n: u32) -> u32 {
    let mut product = 1;
    for i in 1..=n {
        product *= dbg!(i);
    }
    product
}

fn fizzbuzz(n: u32) -> u32 {
    todo!()
}

fn main() {
    let n = 4;
    println!("{n}! = {}", factorial(n));

    print!("asd: {}", fizzbuzz(23))
} */

// Funciones
/* fn gcd(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn main() {
    println!("gcd: {}", gcd(143, 52));
}

* La última expresión en el cuerpo de una función (o cualquier bloque) se convierte en el valor de retorno.
Simplemente omita el; al final de la expresión.

* La palabra clave return se puede usar para una devolución anticipada, pero la forma de "valor básico"
es idiomática al final de una función (refactorice gcd para usar una devolución).

* No se admite la sobrecarga: cada función tiene una implementación única.
*/

// Blocks and Scopes
/* fn main() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}")
    }

    println!("after: {a}")
}

fn main0() {
    let z = 13;
    let x = {
        let y = 10;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}")
} // Si la última expresión (z - y) termina con ;, el tipo y el valor resultante será ().
* El sombreado es diferente de la mutación, porque después del sombreado las ubicaciones de memoria de ambas
variables existen al mismo tiempo. Ambos están disponibles con el mismo nombre,
dependiendo de dónde lo uses en el código.
* Una variable de sombreado puede tener un tipo diferente.
 */

// break y continue
/* fn main() {
    let (mut a, mut b) = (100, 52);
    let result = loop {
        if a == b {
            break a;
        }
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    };
    println!("{result}");
}

fn main() {
    'outer: for x in 1..5 {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
} */

// loop
/* fn main() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
} */

// for
/* fn main() {
    for x in 1..=5 {
        // Con el = incluye al 5
        println!("x: {x}");
    }
} */

// while
/* fn main() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
} */

// Condicionales
/* fn main0() {
    let x = 10;
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}

fn main() {
    let x = 10;
    let size = if x < 20 { "small" } else { "large" };
    println!("number size: {}", size);
} */

// Exercise: Fibonacci
/* fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    let n = 6;
    println!("fib(n) ={}", fib(n));
} */

// Inferencia de tipos
/* fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main0() {
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
    // takes_u32(y);
}

fn main() {
    let x = 3.14;
    let y = 20;
    assert_eq!(x, y);
    // ERROR: no implementation for `{float} == {integer}`
} */

// Strings
/* fn main() {
    let greeting: &str = "Greetings";
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);
    println!("final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    // println!("{:?}", &sentence[5..15]);
    // println!(r#"<a href="link.html">link</a>"#); --> Cadena sin formato
    // println!("<a href=\"link.html\">link</a>");
} */

// Aritmética
/* fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("result: {}", interproduct(120, 100, 248));
} */

// Declaración de Variables
/* fn main() {
    let mut x: i8 = 3;
    // let _y: u8 = 5; // _ para variables no usadas
    // let z = 123i8;
    // let z: f32 = 5.2645956454984;
    // let a: char = 's';
    // let b: bool = false;
    // let c = 1000 + 10_000; = 11000

    println!("x: {x}");
    x = 20;
    println!("x: {x}");
} */
