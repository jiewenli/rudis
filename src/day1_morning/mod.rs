pub fn exam() {

    println!("=====> day1_morning start");
     // scalar type

     let num : u32 = 23;

     let rand : i64 = 64;
 
     println!("num = {}, rand ={}", num, rand);
 
     //cinpound Type
 
     let arr : [i32;2] = [1,2];
     println!("{:?}", arr);
 
     let some_tuples : (u32, &str) = (0, "zhangsan");
 
     println!("some_tuples.0={:?},some_tuples.1={:?}", some_tuples.0, some_tuples.1);
 
     //refreences
 
     let mut x = 2;
     let ref_x : &i32 = &x;
     println!("ref = {}", ref_x);
     let ref_mut_x : &mut i32 = &mut x;
     println!("ref_mut_x = {}", ref_mut_x);
 
 // 空指针（null pointer）：指针值为Null；
 // 野指针（wild pointer）：未经初始化的“垃圾值”地址；
 // 悬挂指针（dangling pointer）：指向已经释放的地址；
 
     //Slices
 
     let mut larger_coll :[u32; 10] = [0,1,2,3,4,5,6,7,8,9];
 
     larger_coll[1] = 10;
 
     let slices_a = &larger_coll[2..larger_coll.len()];
     let slices_b = &larger_coll[..larger_coll.len()-2];
     let slices_c = &larger_coll[..];
     let slices_d = &larger_coll[1..];
 
     println!("slices_a = {:?}", slices_a);
     println!("slices_b = {:?}", slices_b);
     println!("slices_c = {:?}", slices_c);
     //error, cannot assign to `larger_coll[_]` because it is borrowed
     // larger_coll[1] = 1;
     println!("slices_d = {:?}", slices_d);
     larger_coll[1] = 1;
 
     println!("larger_coll = {:?}", larger_coll);
 
     // &str an immutable reference to a string slice.
     // String a mutable string buffer.

     //Rust 的 String 类型是字节向量的包装器。与 Vec<T> 一样，它是owned的。

     let str_one = "hello";

     let mut string_one = String::from("hello");
     string_one.push(' ');
     string_one.push('w');
     string_one.push('o');
     string_one.push_str("rld");

     println!("str_one = {}", str_one);
     println!("string_one (string) = {}", string_one);
     println!("string_one (str) = {}", string_one.as_str());

     //functions 

 
     println!("=====> day1_morning end");
}