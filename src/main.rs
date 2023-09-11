struct Interval {
    pub start: usize,
    pub end: usize,
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

            } else {

            }
        }
        return results_list;
    }


}


fn main(){
    let v = [1,2,3,4,5];
    let i = v.partition_point(|&x| x < 0);
    println!("{i}");
}



