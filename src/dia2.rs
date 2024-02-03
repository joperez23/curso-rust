// Cierres
/* fn apply_with_log(func: impl FnOnce(i32) -> i32, input: i32) -> i32 {
    println!("Calling function on {input}");
    func(input)
}

fn main() {
    let add_3 = |x| x + 3; // : impl Fn(i32) -> i32
    println!("add_3: {}", apply_with_log(add_3, 10));
    println!("add_3: {}", apply_with_log(add_3, 20));

    let mut v: Vec<i32> = Vec::new();
    let mut accumulate /* : impl FnMut(i32) -> i32 */ = |x: i32| {
        v.push(x);
        v.iter().sum::<i32>()
    };
    println!("accumulate: {}", apply_with_log(&mut accumulate, 4));
    println!("accumulate: {}", apply_with_log(&mut accumulate, 5));

    let multiply_sum /* : impl FnOnce(i32) -> i32 */ = |x| x * v.into_iter().sum::<i32>();
    println!("multiply_sum: {}", apply_with_log(multiply_sum, 3));
} */

// El trait Default
/* #[derive(Debug, Default)]
struct Derived {
    x: u32,
    y: String,
    z: Implemented,
}

#[derive(Debug)]
struct Implemented(String);

impl Default for Implemented {
    fn default() -> Self {
        Self("John Smith".into())
    }
}

fn main() {
    let default_struct = Derived::default();
    println!("{default_struct:#?}");

    let almost_default_struct =
        Derived { y: "Y is set!".into(), ..Derived::default() };
    println!("{almost_default_struct:#?}");

    let nothing: Option<Derived> = None;
    println!("{:#?}", nothing.unwrap_or_default());
} */

// Write
/* use std::io::{Result, Write};

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

fn main() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {:?}", buffer);
    Ok(())
} */

// Read
/* use std::io::{BufRead, BufReader, Read, Result};

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn main() -> Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("lines in file: {}", count_lines(file));
    Ok(())
} */

// Casting con as
/* fn main() {
    let value: i64 = 1000;
    println!("as u16: {}", value as u16);
    println!("as i16: {}", value as i16);
    println!("as u8: {}", value as u8);
} */

// From e Into
/* fn main() {
    let s: String = "hello".into();
    let addr: std::net::Ipv4Addr = [127, 0, 0, 1].into();
    let one: i16 = true.into();
    let bigger: i32 = 123_i16.into();
    println!("{s}, {addr}, {one}, {bigger}");
}

fn main() {
    let s = String::from("hello");
    let addr = std::net::Ipv4Addr::from([127, 0, 0, 1]);
    let one = i16::from(true);
    let bigger = i32::from(123_i16);
    println!("{s}, {addr}, {one}, {bigger}");
} */

// Iteradores
/* #[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let p1 = Point { x: 10, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{:?} + {:?} = {:?}", p1, p2, p1 + p2);
} */

// HashMap
/* use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn".to_string(), 207);
    page_counts.insert("Grimms' Fairy Tales".to_string(), 751);
    page_counts.insert("Pride and Prejudice".to_string(), 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    // Use the .entry() method to insert a value if nothing is found.
    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");

    // let pc1 = page_counts
    //     .get("Harry Potter and the Sorcerer's Stone")
    //     .unwrap_or(&336);
    // let pc1 = page_counts
    //     .entry("The Hunger Games".to_string())
    //     .or_insert(374);

    // println!("pcX {:#?}", pc1);
    // println!("{page_counts:#?}");

    let page_counts = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);

    print!("{page_counts:#?}")
} */

// Vec
/* fn main() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("v1: len = {}, capacity = {}", v1.len(), v1.capacity());

    let mut v2 = Vec::with_capacity(v1.len() + 1);
    v2.extend(v1.iter());
    v2.push(9999);
    println!("v2: len = {}, capacity = {}", v2.len(), v2.capacity());

    // Canonical macro to initialize a vector with elements.
    let mut v3: Vec<i8> = vec![0, 0, 1, 2, 3, 4];

    // Retain only the even elements.
    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");

    // Remove consecutive duplicates.
    v3.dedup();
    println!("{v3:?}");
    println!("Vamos {} o {:?}", v3[1], v3.get(2));
} */

// String
/* fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("s1: len = {}, capacity = {}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.len() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("s2: len = {}, capacity = {}", s2.len(), s2.capacity());

    let s3 = String::from("🇨🇭");
    println!("s3: len = {}, number of chars = {}", s3.len(), s3.chars().count());
} */

// Option, Result
/*use std::fs::File;
use std::io::Read;

 fn main() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }
} */

// Duration
/* fn main() {
    let name = "Löwe 老虎 Léopard Gepardi";
    let mut position: Option<usize> = name.find('é');
    println!("find returned {position:?}");
    assert_eq!(position.unwrap(), 14);
    position = name.find('Z');
    println!("find returned {position:?}");
    assert_eq!(position.expect("Character not found"), 0);
} */

// impl Trait
/* // Syntactic sugar for:
//   fn add_42_millions<T: Into<i32>>(x: T) -> i32 {
fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: u32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("debuggable: {debuggable:?}");
} */

// Trait Bounds
/* //  fn duplicate<T: Clone>(a: T) -> (T, T) {
//     (a.clone(), a.clone())
// }

// struct NotClonable;
fn duplicate<T>(a: T) -> (T, T)
where
    T: Clone,
{
    (a.clone(), a.clone())
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
} */

// Tipos de Datos Genéricos
/* #[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }

    // fn set_x(&mut self, x: T)
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
    println!("coords: {:?}", integer.coords());
    let p = Point { x: 5, y: 10.0 };
} */

// Funciones Externas
/* /// Pick even or odd depending on the value of n.
fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
}

fn main() {
    println!("picked a number: {:?}", pick(97, 222, 333));
    println!("picked a tuple: {:?}", pick(28, ("dog", 1), ("cat", 2)));
} */

// Objetos Trait
/* struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog {
            name: String::from("Fido"),
            age: 5,
        }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
} */

// Derivación de Traits
/* #[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default(); // Default trait adds default constructor.
    let mut p2 = p1.clone(); // Clone trait adds clone method.
    p2.name = String::from("EldurScrollz");
    // Debug trait adds support for printing with {:?}.
    println!("{:?} vs. {:?}", p1, p2);
} */

// Traits
/* struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("Oh you're a cutie! What's your name? {}", self.talk());
    }
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn main() {
    let captain_floof = Cat { lives: 9 };
    let fido = Dog {
        name: String::from("Fido"),
        age: 5,
    };

    captain_floof.greet();
    fido.greet();
} */

// Métodos
/* #[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // No receiver, a static method
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    // Exclusive borrowed read-write access to self
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // Shared and read-only borrowed access to self
    fn print_laps(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("Lap {idx}: {lap} sec");
        }
    }

    // Exclusive ownership of self
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("Race {} is finished, total lap time: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
 */

// let else rewritten version
/* fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
  let Some(s) = maybe_string else {
      return Err(String::from("got None"));
  };

  let Some(first_byte_char) = s.chars().next() else {
      return Err(String::from("got empty string"));
  };

  let Some(digit) = first_byte_char.to_digit(16) else {
      return Err(String::from("not a hex digit"));
  };

  return Ok(digit);
} */

// while-let
/* fn main() {
  let mut name = String::from("Comprehensive Rust 🦀");
  while let Some(c) = name.pop() {
      println!("character: {c}");
  }
  // (There are more efficient ways to reverse a string!)
} */

// let else expressions
/* fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
  let s = if let Some(s) = maybe_string {
      s
  } else {
      return Err(String::from("got None"));
  };

  let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
      first_byte_char
  } else {
      return Err(String::from("got empty string"));
  };

  if let Some(digit) = first_byte_char.to_digit(16) {
      Ok(digit)
  } else {
      Err(String::from("not a hex digit"))
  }
}

fn main() {
  println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
} */

// Expresiones if let
/* fn sleep_for(secs: f32) {
  let dur = if let Ok(dur) = std::time::Duration::try_from_secs_f32(secs) {
      dur
  } else {
      std::time::Duration::from_millis(500)
  };
  std::thread::sleep(dur);
  println!("slept for {:?}", dur);
}

fn main() {
  sleep_for(-10.0);
  sleep_for(0.8);
} */

// Desestructurando Enums
/* enum Result {
    Ok(i8),
    Err(String),
    // Suc(String),
}

fn divide_in_two(n: i8) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("cannot divide {n} into two equal parts"))
    }
}

fn main() {
    let n = 99;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} divided in two is {half}"),
        Result::Err(msg) => println!("sorry, an error happened: {msg}"),
    }
}

struct Foo {
    x: (u32, u32),
    y: u32,
}

#[rustfmt::skip]
fn main0() {
  let foo = Foo { x: (1, 2), y: 3 };
  match foo {
      Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
      Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
      Foo { y, .. }        => println!("y = {y}, other fields were ignored"),
  }
} */
