fn main() {
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



}
