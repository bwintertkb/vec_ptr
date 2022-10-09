use vec_ptr::{vec_ptr, VecPtr};

fn main() {
    let mut v = VecPtr::<i32>::default();
    v.push(1);
    v.push(2);
    v.push(3);
    v.insert(1, 99);
    v.update(2, 4);
    unsafe {
        println!("UNSAFE 1: {:?}", v);
        let a = v.get_mut(0).unwrap();
        *a = 200;
        println!("UNSAFE 1: {:?}", v.get_mut(0));
    }
    println!("UNSAFE 1: {:?}", v);
    v.swap(0, 1);
    println!("UNSAFE 2: {:?}", v);
    v.remove(0);
    println!("UNSAFE 3: {:?}", v[0]);

    for p in v.iter().rev() {
        println!("POINTER: {:?}", p);
    }
    // println!("{:?}", v.get_val(0));
    // println!("{:?}", v.get_val(1));
    //let vv: VecPtr<f32> = vec_ptr!(1, 2, 3);
    //println!("VV: {:?}", vv);

    //let ptr = v.get(0);
    // unsafe {
    // println!("UNSAFE 2: {:?}", v);
    // println!("UNSAFE 2: {:?}", v.get_val(0));
    // }
}
