use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    //////////////////
    // creating new //
    //////////////////

    // appending to mutable map

    let mut scores = HashMap::<String, u32>::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // collecting tuples

    let keys = vec!["Blue", "Yellow"];
    let values = vec![10, 50];
    let scores1: HashMap<&str, u32> = keys.iter().cloned().zip(values).collect();

    let scores2: HashMap<&str, u32> = [("Blue", 10), ("Yellow", 50)].iter().cloned().collect();
    assert_eq!(scores1, scores2);

    ///////////////
    // ownership //
    ///////////////

    let key = String::from("favorite color");
    let value = String::from("orange");

    let mut map = HashMap::new();
    map.insert(key, value); // key, value are moved,
                            // owned by map,
                            // and now invalid
                            // (stack-stored values like u32 can be copied)

    // if we insert references, we have to use lifetimes (more on that later!)

    ///////////////
    // accessing //
    ///////////////

    assert_eq!(scores.get(&String::from("Blue")), Some(&10));
    assert_eq!(scores.get(&String::from("Blue")).unwrap_or(&0), &10);

    ///////////////
    // iterating //
    ///////////////

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let tuples: &Vec<String> = &scores1
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect();

    // sometimes will fail b/c sorting:
    // assert_eq!(tuples, &vec!["Blue: 10", "Yellow: 50"]);

    // if we want to ignore sorting, we can do this:
    assert_eq!(
        HashSet::<&String>::from_iter(tuples),
        HashSet::<&String>::from_iter(vec![&"Blue: 10".to_string(), &"Yellow: 50".to_string()])
    );

    //////////////
    // updating //
    //////////////

    // overwriting

    let mut scores3 = HashMap::new();
    scores3.insert(String::from("Blue"), 100);
    scores3.insert(String::from("Blue"), 200);
    assert_eq!(format!("{:?}", scores3), String::from("{\"Blue\": 200}"));

    // updating if empty

    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);
    scores4.entry(String::from("Blue")).or_insert(1000);
    scores4.entry(String::from("Yellow")).or_insert(50);

    assert_eq!(scores4.get(&String::from("Blue")), Some(&10));
    assert_eq!(scores4.get(&String::from("Yellow")), Some(&50));

    // updating based on last value

    let text = "hello world wonderful world";
    let mut scores5 = HashMap::new();

    for word in text.split_whitespace() {
        let count = scores5.entry(word).or_insert(0); // returns a mutable reference
        *count += 1;
    }

    assert_eq!(scores5.get("hello"), Some(&1));
    assert_eq!(scores5.get("world"), Some(&2));
    assert_eq!(scores5.get("wonderful"), Some(&1));
}
