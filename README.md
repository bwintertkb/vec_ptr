# VecPtr

A normal vector, except that it operates on raw pointers and so is a little bit more unsafe than a normal vector...

```
[dependencies]
vec_ptr = "0.1.0"
```

### Example

A little example on how one can use a VecPtr struct...

```
use ::vec_ptr::VecPtr;
use vec_ptr::vec_ptr;

fn main() {
    let macro_vec_ptr: VecPtr<i32> = vec_ptr!(1, 2, 3);
    for v in macro_vec_ptr.iter() {
        println!("{:?}", v);
    }

    let mut struct_vec_ptr: VecPtr<u32> = VecPtr::new();
    struct_vec_ptr.push(9);
}
```

### License

`vec_ptr` is distributed under the terms of the MIT license.
