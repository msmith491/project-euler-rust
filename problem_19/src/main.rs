use std::collections::HashMap;

fn main() {

    fn is_leap_year(y: isize) -> bool {
        if y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) {
            return true;
        }
        false
    }

    fn day_of_week(d: isize) -> String {
        let mut dow = HashMap::new();
        dow.insert(0, "monday");
        dow.insert(1, "tuesday");
        dow.insert(2, "wednesday");
        dow.insert(3, "thursday");
        dow.insert(4, "friday");
        dow.insert(5, "saturday");
        dow.insert(6, "sunday");

        dow.get(&(d % 7)).unwrap().to_string()
    }

    fn total_first_sundays(y: isize, d: isize, start: isize) -> isize {

        if y < 1900 {
            panic!("The year needs to be >= 1900");
        }
        let mut sun_count = 0;
        let mut total_days = 0;
        let iter = y - start;
        let months = vec![("jan", 31),
                          ("feb", 28),
                          ("mar", 31),
                          ("apr", 30),
                          ("may", 31),
                          ("jun", 30),
                          ("jul", 31),
                          ("aug", 31),
                          ("sep", 30),
                          ("oct", 31),
                          ("nov", 30),
                          ("dec", 31)];
        for year in 0..iter {
            for (month, days) in months.clone() {
                if day_of_week(total_days + 1) == "sunday" {
                    sun_count = sun_count + 1;
                }
                if month == "feb" {
                    if is_leap_year(year + start) {
                        total_days = total_days + days + 1;
                    } else {
                        total_days = total_days + days;
                    }
                } else {
                    total_days = total_days + days;
                }

            }
        }
        // total_days + d - 1
        sun_count
    }


    fn month_for_day_in_year(y: isize, d: isize) -> String {
        unimplemented!();

    }

    println!("{}", total_first_sundays(2001, 0, 1901));

}
