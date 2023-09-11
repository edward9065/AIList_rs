use std::fmt;

struct Interval {
    pub start: usize,
    pub end: usize,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

struct AIList {
    starts: Vec<usize>, 
    ends: Vec<usize>,
    max_ends: Vec<usize>,
}

impl AIList {
    fn new(mut intervals: Vec<Interval>) -> AIList {
        // in the future, clone and sort...
        intervals.sort_by_key(|key| key.start);
       
        let mut starts: Vec<usize> = Vec::new();
        let mut ends: Vec<usize> = Vec::new();
        let mut max_ends: Vec<usize> = Vec::new();

        let mut max: usize = 0; 

        for interval in intervals.iter() {
            max = if max > interval.end { max } else { interval.end };
            starts.push(interval.start);
            ends.push(interval.end);
            max_ends.push(max); 
        }

        AIList {
            starts, 
            ends,
            max_ends
        }
    }

    fn query(&self, interval: &Interval) -> Vec<Interval> {
        let mut results_list: Vec<Interval> = Vec::new();
        let  mut i = self.starts.partition_point(|&x| x < interval.end);
      
        while i > 0 {
            i-=1;
            if interval.start > self.ends[i] { //this means that there is no interssection
                if interval.start > self.max_ends[i] { //there is no further intersection
                    return results_list;
                }
            } else {
                results_list.push(Interval {
                    start: self.starts[i],
                    end: self.ends[i],
                })
            }
        }
        return results_list;
    }


}


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

    let ai_list = AIList::new(intervals);
    let hi = ai_list.query(&Interval {start: 18, end: 20});
    for element in hi.iter() {
        print!("{}, ", element);
    }
}



