use vec_ptr::VecPtr;

fn main() {
    let mut v = VecPtr::<i32>::default();
    v.push(1);
    v.push(2);
    v.push(3);
    v.update(2, 4);
    unsafe {
        println!("UNSAFE 1: {:?}", v);
        println!("UNSAFE 1: {:?}", v.get_val(0));
    }
    v.swap(0, 1);
    println!("{:?}", v.get_val(0));
    println!("{:?}", v.get_val(1));

    let vv: Vec<f32> = Vec::new();

    let ptr = v.get_val(0);
    unsafe {
        println!("UNSAFE 2: {:?}", v);
        println!("UNSAFE 2: {:?}", v.get_val(0));
    }
}