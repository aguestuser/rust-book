fn main() {

    /*************
     * mutability
     *************/
    
    // works:
    let mut x = 5;
    println!("x has value: {}", x);
    x = 6;
    println!("x has value: {}", x);

    // won't work (b/c reassigning immutable value):
    let y = 5;
        
    // y= 6 BOOM!

    // will work (b/c shadowing):
    let y = y + 1;
    println!("y has value: {}", y);

    /***********
     * numbers *
     ***********/

    // type annotations needed for parsing number
    let _guess: u32 = "42".parse().expect("Not a number");


    let _n0 = 42u8; // u8
    let _n1 = 42; // u32
    let _n2 = 0xff;
    let _n3 = 0b1111_00000;
    let _n4 = 100_000;
    let _n5 = b'A';

    let _n6 = 2.0; // f64
    let _n7: f32 = 2.0; // f32

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    /***********
     * tuples
     ***********/

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    /***********
     * arrays
     ***********/

    let arr = [1,2,3];
    let _first = arr[0];
    // let _blah = arr[10]; // => panic!

    /**************
     * functions
     **************/

    fn greet(name: String){
        println!("hello {}!", name);
    }
    

    // expressions
    
    let _foo = {
        let x = 1;
        x + 1 // note the omitted semicolon!
    }; // _foo == 2

    // implicit return
    fn identity(a: u32) -> u32 {
        // a; <- compile error!
        a // works
    }

    /***************
     *control flow
     ***************/

    // if is an expression!

    let _num = if 1 == 1 { 1 } else { 0 };
    
    
   for elem in [0, 1, 2].iter(){
        println!("i see {}!", elem);
    }

    for n in (1..3).rev() {
        println!("{}...", n);
    }
    println!("BLAST OFF!");
 }
