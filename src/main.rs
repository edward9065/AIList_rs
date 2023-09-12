use augmented_interval_list::interval::Interval as Interval;
use augmented_interval_list::ailist::AIList;
fn main() {
    let intervals = vec![
        Interval { start: 1, end: 2 },
        Interval { start: 3, end: 8 },
        Interval { start: 5, end: 7 },
        Interval { start: 7, end: 20},
        Interval { start: 9, end: 10},
        Interval { start: 13, end: 15},
        Interval { start: 15, end: 16},
        Interval { start: 19, end: 30},
        Interval { start: 22, end: 24},
        Interval { start: 24, end: 25},
        Interval { start: 26, end: 28},
        Interval { start: 32, end: 38},
        Interval { start: 34, end: 36},
        Interval { start: 38, end: 40},
    ];

    let ai_list = AIList::new(intervals, 3);
    let interval = Interval {start: 13, end: 22};
    


    let hi = ai_list.query(&interval);
    println!("Interval: {}", interval);
    for element in hi.iter() {
        print!("{}, ", element);
    }
    ai_list.print();
}



