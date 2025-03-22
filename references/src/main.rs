fn main() {
    let mut s = String::from("hello");
    let r3 = &mut s; // no problem

    r3.push_str(" world!");

    println!("{r3}");
    println!("{s}");

}
