//use std::marker::PhantomData;

trait Nat {}
struct Z;
struct Succ<N: Nat> {
    //runtime link to previous cell
    _prev: N,
}

//witness
impl Nat for Z {}
impl<T: Nat> Nat for Succ<T> {}

// Example: One is the successor of Zero
type One = Succ<Z>;

// Example: Two is the successor of One
type Two = Succ<One>;

// Example: Three is the successor of Two
type Three = Succ<Two>;

pub trait NatToString {
    fn to_string(&self) -> String;
}

// Base case: Zero converts to "0"
impl NatToString for Z {
    fn to_string(&self) -> String {
        "Z".to_string()
    }
}

// Recursive case: The successor of N converts to the concatenation of "1" and N's string
impl<N: Nat + NatToString> ToString for Succ<N> {
    fn to_string(&self) -> String {
        format!("S{}", self._prev.to_string())
    }
}

fn main() {
    let x: Succ<Z> = Succ { _prev: Z {} };
    println!("{}", x.to_string());
}
