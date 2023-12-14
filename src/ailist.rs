use crate::interval::Interval;
use num_traits::{
    identities::{one, zero},
    PrimInt, Unsigned,
};
pub struct AIList<T: PrimInt + Unsigned + Ord + Clone + Send + Sync> {
    starts: Vec<T>, 
    ends: Vec<T>,
    max_ends: Vec<T>,
    header_list: Vec<usize>,
}

impl<T: PrimInt + Unsigned + Ord + Clone + Send + Sync + std::fmt::Display> AIList<T> {
    pub fn new(intervals: &mut Vec<Interval<T>>, minimum_coverage_length: usize) -> AIList<T> {
        // in the future, clone and sort...
        intervals.sort_by_key(|key| key.start);
       
        let mut starts: Vec<T> = Vec::new();
        let mut ends: Vec<T> = Vec::new();
        let mut max_ends: Vec<T> = Vec::new();
        let mut header_list: Vec<usize> = vec![0];

        loop {
            
            let mut results = Self::decompose(intervals, minimum_coverage_length);
            
            starts.append(&mut results.0);
            ends.append(&mut results.1);
            max_ends.append(&mut results.2);

            *intervals = results.3;

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

    fn decompose(intervals: &Vec<Interval<T>>, minimum_coverage_length: usize) -> (Vec<T>, Vec<T>, Vec<T>, Vec<Interval<T>>) {
        // look at the next minL*2 intervals
        let mut starts: Vec<T> = Vec::new();
        let mut ends: Vec<T> = Vec::new();
        let mut max_ends: Vec<T> = Vec::new();
        let mut l2: Vec<Interval<T>> = Vec::new();
        
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

        let mut max: T = ends[0]; 

        for end in ends.iter() {
            max = if max > *end { max } else { *end };
            max_ends.push(max); 
        }

        (starts, ends, max_ends, l2)
    }

    fn query_slice(interval: &Interval<T>, starts: &[T], ends: &[T], max_ends: &[T]) -> Vec<Interval<T>>{
        let mut results_list: Vec<Interval<T>> = Vec::new();
        // problem here
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

    pub fn query(&self, interval: &Interval<T>) -> Vec<Interval<T>> {
        let mut results_list: Vec<Interval<T>> = Vec::new();
        
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
        // now do the last decomposed ailist
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

  


    pub fn print(&self) {
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