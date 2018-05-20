use std::collections::HashMap;

fn main() {
    /////////////////////////
    // CHAPTER 8 EXERCISES //
    /////////////////////////

    ///////////////////
    // integer stats //
    ///////////////////

    // tests

    assert_eq!(mean(&vec![1, 2, 3]), 2.0);
    assert_eq!(median(&mut vec![1, 2, 3]), 2);
    assert_eq!(mode(&vec![1, 2, 2, 3]), Some(2));

    // code

    fn mean(nums: &Vec<u32>) -> f32 {
        (nums.iter().fold(0, |acc, n| acc + n) as f32) / (nums.len() as f32)
    }

    fn median(nums: &mut Vec<u32>) -> u32 {
        nums.sort();
        nums[nums.len() / 2]
    }

    fn mode(nums: &Vec<u32>) -> Option<u32> {
        let counts = count_occurences(nums);
        counts.values().cloned().max()
    }

    fn count_occurences(nums: &Vec<u32>) -> HashMap<&u32, u32> {
        // hmmm... i wonder if there's a more functional way to do this?
        let mut counts = HashMap::new();
        for n in nums {
            let count = counts.entry(n).or_insert(0);
            *count += 1;
        }
        counts
    }

    ///////////////
    // pig-latin //
    ///////////////

    // tests

    assert_eq!(
        pig_latinize(String::from("first")),
        String::from("irst-fay")
    );
    assert_eq!(
        pig_latinize(String::from("apple")),
        String::from("apple-hay")
    );

    // code

    fn pig_latinize(word: String) -> String {
        let (hd, tl) = get_head_and_tail(&word[..]);
        if is_consonant(hd) {
            format!("{}-{}ay", tl, hd)
        } else {
            format!("{}-hay", word)
        }
    }

    fn get_head_and_tail(word: &str) -> (&str, &str) {
        (&word[..1], &word[1..])
    }

    const CONSONANTS: [char; 21] = [
        'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w',
        'x', 'y', 'z',
    ];

    fn is_consonant(str: &str) -> bool {
        let first_char = str.chars().next().unwrap();
        CONSONANTS.contains(&first_char)
    }

    ///////////////
    // directory //
    ///////////////

    // tests

    let directory = build_directory(vec![
        "Add Connor to Joy",
        "Add Austin to Planmaking",
        "Add Ziggy to Gadgets",
        "Add Jenny to Planmaking",
        "Add Emma to Planmaking",
        "Add Monica to Joy",
        "Add Kelley to Gadgets",
    ]);

    assert_eq!(
        directory,
        [
            (
                String::from("Joy"),
                vec![String::from("Connor"), String::from("Monica")]
            ),
            (
                String::from("Planmaking"),
                vec![
                    String::from("Austin"),
                    String::from("Emma"),
                    String::from("Jenny"),
                ],
            ),
            (
                String::from("Gadgets"),
                vec![String::from("Kelley"), String::from("Ziggy")],
            )
        ].iter()
            .cloned()
            .collect()
    );

    assert_eq!(
        people_in_department(&directory, String::from("Joy")),
        String::from("Connor, Monica")
    );

    assert_eq!(
        people_in_department(&directory, String::from("Non-existent")),
        String::from("Nobody")
    );

    assert_eq!(
        people_by_department(&directory),
        String::from(
            "Gadgets: Kelley, Ziggy\n\
             Joy: Connor, Monica\n\
             Planmaking: Austin, Emma, Jenny"
        )
    );

    //code

    type Directory = HashMap<String, Vec<String>>; // would prefer &str but don't understand lifetimes yet!

    fn build_directory(cmds: Vec<&str>) -> Directory {
        let mut dir = Directory::new();
        for cmd in cmds {
            process_cmd(&mut dir, cmd);
        }
        for (_, v) in &mut dir {
            v.sort()
        }
        dir
    }

    fn process_cmd(dir: &mut Directory, cmd: &str) {
        let pair = cmd[4..].split(" to ").collect::<Vec<&str>>();
        let (name, dept) = (String::from(pair[0]), String::from(pair[1]));
        let names = dir.entry(dept).or_insert(Vec::new());
        names.push(name);
    }

    fn people_in_department(dir: &Directory, dpt: String) -> String {
        dir.get(&dpt)
            .get_or_insert(&vec![String::from("Nobody")])
            .join(", ")
    }

    fn people_by_department(dir: &Directory) -> String {
        let mut dept_listings: Vec<String> = dir
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v.join(", ")))
            .collect();
        dept_listings.sort();
        dept_listings.join("\n")
    }
}
