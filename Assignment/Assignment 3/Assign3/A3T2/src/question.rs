use super::*;
use hamcrest::*;

#[test]
fn name_is_String() {
    let player_one = player::Player {
        id: 001,
        first_name: String::from("Tim"),
        last_name: String::from("White"),
    };

    let player_two = player::Player {
        id: 002,
        first_name: String::from("Mike"),
        last_name: String::from("Thompson"),
    };

    assert_that!(player_one.first_name, is(type_of::<String>()));
    assert_that!(player_one.last_name, is(type_of::<String>()));
}
#[test]
fn all_the_same(){
    let player_one = player::Player {
        id: 001,
        first_name: String::from("Tim"),
        last_name: String::from("White"),
    };

    let player_two = player::Player {
        id: 001,
        first_name: String::from("Tim"),
        last_name: String::from("White"),
    };

    assert_that!(player_one.id,is(equal_to(player_two.id)));
    assert_that!(player_one.first_name,is(equal_to(player_two.first_name)));
    assert_that!(player_one.last_name,is(equal_to(player_two.last_name)));
}
