use regex::Regex;

pub struct RegexSliceIterator<'p, 'l> {
    pattern: &'p Regex,
    line: &'l str,
    current_pos: usize,
}

pub struct RegexSliceMatch<'l> {
    line: &'l str,
    start: usize,
    end: usize,
}

impl<'l> RegexSliceMatch<'l> {
    pub fn as_str(&self) -> &'l str {
        unsafe { self.line.get_unchecked(self.start..self.end) }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'p, 'l> RegexSliceIterator<'p, 'l> {
    pub fn new(pattern: &'p Regex, line: &'l str) -> RegexSliceIterator<'p, 'l> {
        RegexSliceIterator {
            pattern: pattern,
            line: line,
            current_pos: 0,
        }
    }
}

impl<'p, 'l> Iterator for RegexSliceIterator<'p, 'l> {
    type Item = RegexSliceMatch<'l>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos >= self.line.len() {
            return None;
        }

        let maybe_match = unsafe {
            let rem = self.line.get_unchecked(self.current_pos..);
            self.pattern.find(rem)
        };

        match maybe_match {
            Some(m) => {
                let prev_pos = self.current_pos;
                self.current_pos += m.end();
                Some(RegexSliceMatch {
                    line: self.line,
                    start: prev_pos + m.start(),
                    end: prev_pos + m.end(),
                })
            }
            None => {
                self.current_pos = self.line.len();
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let text = "123456789";
        let pattern = regex::Regex::new("(1|3|5|7|9)").expect("Invalid regex");
        let iter = RegexSliceIterator::new(&pattern, text);
        let results:Vec<_> = iter.map(|x| x.as_str()).collect();
        assert_eq!(results, ["1", "3", "5", "7", "9"]);
    }

    #[test]
    fn test_iter_no_matches() {
        let text = "123456789";
        let pattern = regex::Regex::new("(a|b)").expect("Invalid regex");
        let iter = RegexSliceIterator::new(&pattern, text);
        let results:Vec<_> = iter.map(|x| x.as_str()).collect();
        assert_eq!(results.len(), 0);
    }
}
