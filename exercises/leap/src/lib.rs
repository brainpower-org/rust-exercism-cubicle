pub fn is_leap_year(year: u64) -> bool {
    // Solution 1
    // year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)

    // Solution 2
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        _ => false
    }

    // Solution 3
    // struct LeapYear {
    //     _4: bool,
    //     _100: bool,
    //     _400: bool,
    // }

    // let year_struct = LeapYear {
    //     _4: year % 4 == 0,
    //     _100: year % 100 == 0,
    //     _400: year % 400 == 0,
    // };

    // match year_struct {
    //     LeapYear {
    //         _4: true,
    //         _100: true,
    //         _400: true,
    //     } => true,
    //     LeapYear {
    //         _4: true,
    //         _100: false,
    //         _400: false,
    //     } => true,
    //     LeapYear {
    //         _4: true,
    //         _100: true,
    //         _400: false,
    //     } => false,
    //     _ => false,
    // }
}
