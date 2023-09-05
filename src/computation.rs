use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;
use sha256::digest;

pub struct Computation {
    pub max_cpu: usize,
    pub number_range: i32,
    pub check_mask: String,
    pub found_need: usize,
}

impl Computation {
    /// Method to generate vector of tuples (number and its hash sha256)
    ///
    /// # Input:
    ///  - (start_range, end_range) = start number and end number to analyze
    ///  - check_mask = string mask to compare contains suffix
    ///
    /// # Returns
    /// Returns tuples of number and its hash
    ///
    /// # Examples
    ///
    /// ```
    /// let hashes = Computation::get_sha256_digests_with_zero_suffix_ranged(
    ///     1,
    ///     1000,
    ///     "00"
    /// );
    /// # assert_eq!(hashes.len() > 0, true);
    /// ```
    fn get_sha256_digests_with_zero_suffix_ranged(start_range: i32, end_range: i32, check_mask: &str) -> Vec<(i32, String)> {
        let mut out = vec![];
        let suffix_length = check_mask.len();

        for iter in start_range..end_range {
            let sha256 = digest(iter.to_string());
            let check = &sha256[65 - suffix_length - 1..];
            if check.contains(check_mask) {
                out.push((iter, sha256));
            }
        }
        out
    }

    /// Method to initialize first N threads
    /// where N - number of max_cpu in configuration
    ///
    /// # Returns
    /// Returns two things:
    /// - queue with thread joins
    /// - offset of checked number
    ///
    fn initial_threads(&self) -> (
        VecDeque<JoinHandle<Vec<(i32, String)>>>,
        i32
    ) {
        let mut part = 0;
        let mut queue: VecDeque<JoinHandle<Vec<(i32, String)>>> = VecDeque::new();

        // start all threads
        for _ in 0..self.max_cpu {
            let check_mask = self.check_mask.clone();
            let number_range = self.number_range;
            let join = thread::spawn(move || {
                Self::get_sha256_digests_with_zero_suffix_ranged(
                    part + 1,
                    part + number_range,
                    check_mask.as_str(),
                )
            });
            queue.push_back(join);
            part += self.number_range;
        }

        (queue, part)
    }

    /// Method to start search hashes by mask
    ///
    /// # Returns
    /// Returns vector of numbers and its hashes
    ///
    pub fn compute(&self) -> Vec<(i32, String)> {
        let mut hashes: Vec<(i32, String)> = vec![];

        if self.found_need > 0 {
            // start all threads
            let (mut queue, mut part) = self.initial_threads();

            // join first added
            // if founded hashes less than need, start new thread with new range
            while hashes.len() < self.found_need {
                let mut pop = queue.pop_front().unwrap().join().unwrap().clone();
                hashes.append(&mut pop);

                if hashes.len() < self.found_need {
                    let check_mask = self.check_mask.clone();
                    let number_range = self.number_range;
                    let join = thread::spawn(move || {
                        Self::get_sha256_digests_with_zero_suffix_ranged(
                            part + 1,
                            part + number_range,
                            check_mask.as_str())
                    });
                    queue.push_back(join);
                    part += self.number_range;
                }
            }
        }

        hashes.into_iter().take(self.found_need).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let computation = Computation {
            max_cpu: 3,
            number_range: 100,
            check_mask: String::from("00"),
            found_need: 4
        };
        let hashes = computation.compute();
        assert!(!hashes.is_empty() && hashes.len() == computation.found_need);
    }

    #[test]
    fn works_with_no_mask() {
        let computation = Computation {
            max_cpu: 3,
            number_range: 100,
            check_mask: String::from(""),
            found_need: 4
        };
        let hashes = computation.compute();
        assert!(!hashes.is_empty() && hashes.len() == computation.found_need);
    }

    #[test]
    fn works_with_zero_found() {
        let computation = Computation {
            max_cpu: 3,
            number_range: 100,
            check_mask: String::from("00"),
            found_need: 0
        };
        let hashes = computation.compute();
        assert!(hashes.is_empty());
    }
}
