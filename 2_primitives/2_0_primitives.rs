fn main() {
 // Variables can be type annotated
 let logical : bool = true;

 let a_float : f64 = 1.0;
 let an_integer = 5i32 // suffix annotation
 
 // or a default
 let default_float = 3.2; // f64
 let default_integer = 1; // i32

 // type can be inferred from context
 let mut inferred_type = 12 // type i64 is inferred from another line
 inferred_type = 4294967296i64;

 // A mutable variable's value can be changed
 let mut mutable = 12; // Mutable `i32`
 mutable = 21;

 // but we can't change the type of a variable
 mutable = true // error

 // variables can be overwritten with shadowing
 let mutable = true
}