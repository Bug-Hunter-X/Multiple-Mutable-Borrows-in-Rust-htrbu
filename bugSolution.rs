fn main() {
    let mut x = 5;
    { //Use a scope to limit the borrow
        let y = &mut x;
        *y += 1;
    }
    { //Use a scope to limit the borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}