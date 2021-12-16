fn main() {
    let mut groups = [[""; 4]; 6];
    groups[0] = ["Bob", "Carol", "Eric", "Matt"];
    groups[1] = ["Jim", "Lucy", "Terry", "Brenda"];
    groups[2] = ["Susan", "Brad", "Jim", "Matt"];
    groups[3] = ["Sue", "Wendy", "Sam", "Brad"];
    groups[4] = ["Kate", "Jack", "James", "Sydney"];
    groups[5] = ["Mary", "John", "Ricky", "Wendy"];

    let result = search_member(groups, handle_input());
    println!("{:?}", result);
}

// 1. exists? 2. group member? 3. leader?
fn search_member(group_lists: [[&str; 4]; 6], target: String) {
    let target = target.as_str().trim();

    let mut exist_flag = false;
    let mut result_exist = String::new();
    let mut result_info = Vec::new();

    // go over all the lists
    for group_number in 0..group_lists.len() {
        for person_info in group_lists[group_number].iter().enumerate() {
            // save the name and its index as (index, name)
            let (index, &name) = person_info;
            // 1. check whether it's same as the person we want
            if target == name {
                exist_flag = true;
                // 2&3: In which group? Is he/she the group leader?
                if index == 0 {
                    result_info.push(format!("{} is in the No.{} group, and he/she is the leader!", name, group_number)) ;
                } else {
                    result_info.push(format!("{} is in the No.{} group.", name, group_number));
                };
            };
        };
    }

    if exist_flag == true {
        result_exist = "This person exists".to_string();
    } else {
        result_exist = "This person doesn't exist".to_string();
    }

    println!("{}", result_exist);

    for res in result_info {
        println!("{}",res)
    }
}

fn handle_input() -> String {
    println!("please input the name that you want to search:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Can not parse!");
    input
}