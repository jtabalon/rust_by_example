fn main() {
    // Variables can be type annotated
    let logical: bool = true;

    let a_float: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // Suffix annotation

    let default_float = 3.0; // floats default to float64
    let default_integer = 7; // integers default to int32

    // a default can be used
    let mut inferred_type = 12;
    inferred_type = 4294967296i64;

    // Types can be inferred from context
    let mut mutable = 12;
    mutable = 21;


    //Error: The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true;
}
