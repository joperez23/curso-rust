// NO TERMINADO, PARA DESPUÉS

// Referencias colgantes
/* fn main() {
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");
    s2 = String::from("Épale ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[6..];
    println!("s3: {s3}");
} */

// Slices
/* fn main() {
  let mut a: [i32; 6] = [10, 20, 30, 40, 50, 60];
  println!("a: {a:?}");

  let s: &[i32] = &a[2..4];

  println!("s: {s:?}");
} */

// Préstamos (Borrowing) 2
/* fn main() {
    let mut a: i32 = 10;
    let b: &i32 = &a;
    println!("b: {b}");
    {
        let c: &mut i32 = &mut a;
        *c = 20;
    }

    println!("a: {a}");
} */

// Préstamos (Borrowing) 1
/* #[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
} */

// El Trait Drop
/* struct Droppable {
  name: &'static str,
}

impl Drop for Droppable {
  fn drop(&mut self) {
      println!("Dropping {}", self.name);
  }
}

fn main() {
  let a = Droppable { name: "a" };
  {
      let b = Droppable { name: "b" };
      {
          let c = Droppable { name: "c" };
          let d = Droppable { name: "d" };
          println!("Exiting block B");
      }
      println!("Exiting block A");
  }
  drop(a);
  println!("Exiting main");
} */

// Tipos compuestos
/* #[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;
    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
} */
