/// Solution 1
/// ```
/// pub fn is_leap_year(year: u64) -> bool {
///     year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
/// }
/// 
/// # fn process_leapyear_case(year: u64, expected: bool) {
/// #   assert_eq!(leap::is_leap_year(year), expected);
/// # }
/// 
/// process_leapyear_case(1996, true);
/// process_leapyear_case(2015, false);
/// process_leapyear_case(1800, false);
/// process_leapyear_case(2100, false);
/// process_leapyear_case(2000, true);
/// assert_eq!(leap::is_leap_year(1997), false);
/// assert_eq!(leap::is_leap_year(1700), false);
/// assert_eq!(leap::is_leap_year(1800), false);
/// assert_eq!(leap::is_leap_year(1900), false);
/// assert_eq!(leap::is_leap_year(1600), true);
/// assert_eq!(leap::is_leap_year(2000), true);
/// assert_eq!(leap::is_leap_year(2400), true);
/// ```
/// 
/// Solution 3
/// ```
/// pub fn is_leap_year(year: u64) -> bool {
///     struct LeapYear {
///         _4: bool,
///         _100: bool,
///         _400: bool,
///     }
///
///    let year_struct = LeapYear {
///        _4: year % 4 == 0,
///        _100: year % 100 == 0,
///        _400: year % 400 == 0,
///    };
///
///    match year_struct {
///        LeapYear {
///            _4: true,
///            _100: true,
///            _400: true,
///        } => true,
///        LeapYear {
///            _4: true,
///            _100: false,
///            _400: false,
///        } => true,
///        LeapYear {
///            _4: true,
///            _100: true,
///            _400: false,
///        } => false,
///        _ => false,
///    }
/// }
/// 
/// # fn process_leapyear_case(year: u64, expected: bool) {
/// #   assert_eq!(leap::is_leap_year(year), expected);
/// # }
/// 
/// process_leapyear_case(1996, true);
/// process_leapyear_case(2015, false);
/// process_leapyear_case(1800, false);
/// process_leapyear_case(2100, false);
/// process_leapyear_case(2000, true);
/// assert_eq!(leap::is_leap_year(1997), false);
/// assert_eq!(leap::is_leap_year(1700), false);
/// assert_eq!(leap::is_leap_year(1800), false);
/// assert_eq!(leap::is_leap_year(1900), false);
/// assert_eq!(leap::is_leap_year(1600), true);
/// assert_eq!(leap::is_leap_year(2000), true);
/// assert_eq!(leap::is_leap_year(2400), true);
/// ```
pub fn is_leap_year(year: u64) -> bool {
    // Solution 2
    match (year % 4, year % 100, year % 400) {
        (0, 0, 0) => true,
        (0, 0, _) => false,
        (0, _, _) => true,
        _ => false
    }
}
