use crate::optional::USState::Alabama;

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(USState),
}


impl Coin {
    fn value_in_cents(&self) -> i32 {
        match self {
            Coin::Penny => 1,
            Coin::Nickle => 5,
            Coin::Dime => 10,
            Coin::Quarter(_) => 25
        }
    }
    fn print(&self) {
        match self {
            Coin::Quarter(state) => {
                log::info!("coin: {:?}, value: {}, state: {:?}",self, self.value_in_cents(),state)
            }
            _ => log::info!("coin: {:?}, value: {}",self,self.value_in_cents())
        }
    }
}

#[allow(dead_code)]
pub fn run() {
    Coin::Penny.print();
    Coin::Dime.print();
    Coin::Nickle.print();
    Coin::Quarter(Alabama).print();

    let original = Some(5);
    let incremented = increment_by_1(original);
    log::info!("original:{:?}, incremented:{:?}",original,incremented)
}


fn increment_by_1(i: Option<i32>) -> Option<i32> {
    match i {
        Some(i) => Some(i + 1),
        None => None
    }
}

#[allow(dead_code)]
#[derive(Debug)]
enum USState {
    Alaska,
    Alabama,
    Arkansas,
    American,
    Samoa,
    Arizona,
    California,
    Colorado,
    Connecticut,
    DistrictOfColumbia,
    Delaware,
    Florida,
    Georgia,
    Guam,
    Hawaii,
    Iowa,
    Idaho,
    Illinois,
    Indiana,
    Kansas,
    Kentucky,
    Louisiana,
    Massachusetts,
    Maryland,
    Maine,
    Michigan,
    Minnesota,
    Missouri,
    Mississippi,
    Montana,
    NorthCarolina,
    NorthDakota,
    Nebraska,
    NewHampshire,
    NewJersey,
    NewMexico,
    Nevada,
    NewYork,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    PuertoRico,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Virginia,
    VirginIslands,
    Vermont,
    Washington,
    Wisconsin,
    WestVirginia,
    Wyoming,
}