fn main() {
    // declaration
    let _v1: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();
    v3.push(1);
    v3.push(2);
    v3.push(3);

    // value extraction
    let _third: &i32 = &v2[2];
    let _third: Option<&i32> = v2.get(2);

    //let _boom = &v2[1000];
    let _none = v2.get(100);

    // iteration
    for n in &v2 {
        println!("{}", n)
    }

    for n in &mut v3 {
        *n += 1; // we need * to dereference n to change its value
    }

    // list of enums

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.1),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
