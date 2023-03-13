

static mut NAME: &str = "zhangsan";

pub fn exam() {
    println!("=====> day1_afternoon start");

    //static & const
    println!("");
    println!("☆  varibales static & const");

    const PI: f64 = 3.141592654;
    const ONE: u32 = 1;

    println!("PI = {PI}");
    println!("ONE = {ONE}");
    unsafe{
        println!("before NAME = {NAME}");
        changeName();
        println!("after NAME = {NAME}");
    }

    //variables
    //Scopes and Shadowing
    println!("");
    println!("☆  Scopes and Shadowing");

    let a = 10;
    println!("before:{}", a);
    {
        let a = "hello";
        println!("inner scope:{a}");
        let a = true;
        println!("shadowed in inner scope: {a}");
    }
    println!("after:{}", a);

    memoryManagement();
    ownerShip();
    println!("=====> day1_afternoon end");
}

fn changeName() {
    unsafe{ NAME = "lisi";}
}

fn memoryManagement() {
    println!("memory management");

    println!("memory management in Rust");

    println!("Safe and correct like Java, but without a garbage collector.");
    println!("Depending on which abstraction(or combination abstractions) you choose, can be a single unique pointer, reference counted, or atomically reference counted.");

}

fn ownerShip() {
    println!("All variable bindings have a scope where they are valid and it is an error to use a variable outside its scope");
}