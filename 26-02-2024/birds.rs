struct BirdCount { }
impl BirdCount { 
    fn last_week() -> Vec<i32> { 
        Vec::from([1, 2, 5, 0, 7, 8, 4])
    }

    fn today() -> i32 { 
        Self::last_week()[6]
    } 
    
    fn increment_todays_count() -> Vec<i32> { 
        let mut week = Self::last_week();
        week[6] += 1;
        return week;
    }

    fn has_day_without_birds() -> bool { 
        let find = Self::last_week().into_iter().find(| &x| x <= 0);
        if find == None {
            return false;
        } else {
            return true;
        }
    }

    fn count_for_first_days(n: i32) -> i32 {
        let mut x = 0;
        for i in Self::last_week() { 
            x += i;
        }
        return x;
    }

    fn busy_days() -> i32 { 
        let mut x = 0;
        for i in Self::last_week() { 
            if i >=5 { 
                x += 1
            }
        }

        return x;
    }
}

fn main() {
    println!("{:?}", BirdCount::has_day_without_birds());
}
