#[allow(dead_code)]
struct UnPrintable(i32);

#[derive(Debug)]
struct DebugPrintable(i32);

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// all library types are automatically printable with {:?} -> i.e. "Christaian" "Slater" not Christian Slater

fn main() {
    println!("{:?} months in a year", 12);
    // this will print in debug format
    println!(
        "{1:?} {0:?} is the {actor:?} name.",
        "Slater",
        "Christian",
        actor = "actor's"
    );

    // structure is printable
    println!("{:?} has printed..", Structure(2));
    println!("{:?} ugly format?", Deep(Structure(2)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }

    let name = "Dorin";
    let age = 36;

    let dorin = Person { name, age };

    // pretty print with {:#?}
    println!("{:#?}", dorin);

    println!("{0} {1}", dorin.name, dorin.age);
}
