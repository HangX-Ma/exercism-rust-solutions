#[derive(Debug)]
pub struct HighScores<'a> {
    _scores: &'a [u32]
}

impl <'a> HighScores <'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self { _scores: scores }
    }

    pub fn scores(&self) -> &[u32] {
        // unimplemented!("Return all the scores as a slice")
        self._scores
    }

    pub fn latest(&self) -> Option<u32> {
        // unimplemented!("Return the latest (last) score")
        self._scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        // unimplemented!("Return the highest score")
        self._scores.into_iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        // unimplemented!("Return 3 highest scores")
        let mut v = self._scores.to_owned();
        v.sort_unstable();
        v.reverse();
        match v {
            _v if _v.is_empty() => vec![],
            _v if _v.len() < 3 => _v.to_vec(),
            _v => _v.windows(3).next().unwrap().to_vec()
        }
    }
}
