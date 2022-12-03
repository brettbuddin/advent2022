pub fn scores() -> String {
    let mut priorities: Vec<char> = ('a'..='z').collect();
    let upper: Vec<char> = ('A'..='Z').collect();
    priorities.extend(&upper);
    priorities.iter().collect::<String>()
}
