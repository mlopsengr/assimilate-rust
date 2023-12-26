

const LIFE: u32 = 8;
const DEATH: i32 = -28;



// add a main function
fn main() {
    println!("{}", DEATH);
    println!("{}", LIFE);

    return;
}

let mut s1 = String::from("abc");
do_stuff(&mut s1);

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Hi, ");
}
