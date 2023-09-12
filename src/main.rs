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
    header_list: Vec<usize>,
}

impl AIList {
    fn new(mut intervals: Vec<Interval>, minimum_coverage_length: usize) -> AIList {
        // in the future, clone and sort...
        intervals.sort_by_key(|key| key.start);
       
        let mut starts: Vec<usize> = Vec::new();
        let mut ends: Vec<usize> = Vec::new();
        let mut max_ends: Vec<usize> = Vec::new();
        let mut header_list: Vec<usize> = vec![0];

        

        loop {
            
            let mut results = Self::decompose(intervals, minimum_coverage_length);
            
            starts.append(&mut results.0);
            ends.append(&mut results.1);
            max_ends.append(&mut results.2);

            intervals = results.3;

            if intervals.len() == 0 {
                break;
            } else {
                header_list.push(starts.len());
            }
               
        }


        AIList {
            starts, 
            ends,
            max_ends,
            header_list
        }
    }

    fn decompose(intervals: Vec<Interval>, minimum_coverage_length: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<Interval>) {
        // look at the next minL*2 intervals
        let mut starts: Vec<usize> = Vec::new();
        let mut ends: Vec<usize> = Vec::new();
        let mut max_ends: Vec<usize> = Vec::new();
        let mut l2: Vec<Interval> = Vec::new();
        
        for (index, interval) in intervals.iter().enumerate() {
            let mut count = 0;
            for i in 1..(minimum_coverage_length*2) {
                match intervals.get(index+i) {
                    Some(interval2) => if interval.end > interval2.end { count += 1; },
                    None => break,
                }
            }
            if count >= minimum_coverage_length {
                l2.push(Interval {
                    start: interval.start,
                    end: interval.end,
                });
            } else {
                starts.push(interval.start);
                ends.push(interval.end)
            }
        }

        let mut max: usize = 0; 

        for end in ends.iter() {
            max = if max > *end { max } else { *end };
            max_ends.push(max); 
        }

        (starts, ends, max_ends, l2)
    }

    fn query_slice(interval: &Interval, starts: &[usize], ends: &[usize], max_ends: &[usize]) -> Vec<Interval>{
        let mut results_list: Vec<Interval> = Vec::new();
        let mut i = starts.partition_point(|&x| x < interval.end);

        while i > 0 {
            i-=1;
            if interval.start > ends[i] { //this means that there is no intersection
                if interval.start > max_ends[i] { //there is no further intersection
                    return results_list;
                }
            } else {
                results_list.push(Interval {
                    start: starts[i],
                    end: ends[i],
                })
            }
        }
        return results_list;
    }

    fn query(&self, interval: &Interval) -> Vec<Interval> {
        let mut results_list: Vec<Interval> = Vec::new();
        
        for i in 0..(self.header_list.len()-1) {
            results_list.append(
                &mut Self::query_slice(
                    interval, 
                    &self.starts[self.header_list[i]..self.header_list[i+1]],
                    &self.ends[self.header_list[i]..self.header_list[i+1]],
                    &self.max_ends[self.header_list[i]..self.header_list[i+1]],
                )
            );
        }
        // now do the last decomposed AIList
        let i = self.header_list.len()-1;
        results_list.extend(
            Self::query_slice(
                interval, 
                &self.starts[self.header_list[i]..],
                &self.ends[self.header_list[i]..],
                &self.max_ends[self.header_list[i]..],
            )
        );
        
        return results_list;
    }

  


    fn print(&self) {
        println!("");
        for element in self.starts.iter() {
           print!("{}, ", element);
        }
        println!("");
        for element in self.ends.iter() {
            print!("{}, ", element);
        }
        println!("");
        for element in self.max_ends.iter() {
            print!("{}, ", element);
        }
        println!("");
        for element in self.header_list.iter() {
            print!("{}, ", element);
        }
        println!("");


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

    let ai_list = AIList::new(intervals, 3);
    let interval = Interval {start: 1, end: 6};
    let hi = ai_list.query(&interval);
    println!("Interval: {}", interval);
    for element in hi.iter() {
        print!("{}, ", element);
    }
    ai_list.print();
}



