/*
  Formatted print
  ===============
  format!
  print! (io::stdout)
  println!
  eprint! (io::stderr)
  eprintln!
*/

fn main() {
    // ARGUMENTS
    println!("{} days this February", 28);

    // POSITIONAL ARGUMENTS
    println!("{2} {1} {0}", "Dori", "Zola", "Dita");

    // NAMED ARGUMENTS
    println!(
        "{subject} {verb} {noun}",
        noun = "carrots",
        subject = "food",
        verb = "cooks"
    );

    println!("Base 10:          {}", 69);
    println!("Base 2:           {:b}", 69);
    println!("Base 8:           {:o}", 69);
    println!("Base 16:          {:x}", 69);
    println!("Base 16:          {:X}", 69);

    // right-justify text with a width (5)
    println!("{number:>5}", number = 1);
    // pad with zeros
    println!("{number:0<5}", number = 1);
    // can use named argument
    println!("{number:0>width$}", number = 1, width = 5);

    println!("Fixed you, {0} {1}", "Bob", ", by Elly");

    #[allow(dead_code)]
    struct Structure(i32);
    // does not compile as it doesn't implement fmt::Display trait
    // println!("This struct {}", Structure(2));

    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // with precission in formatting (2 decimals)
    let pi: f64 = 3.141592;
    println!("Pi is roughly {pi:.2}");
    println!("Pi is roughly {0:.2}", 3.141592);

    // usize (unsigned, only positive)
    // i32 i34 (signed integers)
    // f32 f64 (signed floating points)
}
