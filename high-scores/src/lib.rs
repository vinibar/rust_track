#[derive(Debug)]
pub struct HighScores<'a> {
    scores_list: &'a [u32],
}

impl<'a> HighScores<'a> {

    pub fn new(scores: &'a [u32]) -> Self {

        HighScores{
          scores_list: scores,   
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores_list
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores_list.last().cloned()
    }

    pub fn personal_best(&self) -> Option<u32> {
       self.scores_list.iter().max().cloned()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut list = self.scores_list.to_vec();
        list.sort();
        list.reverse();
        list.truncate(3);
        list
    }
}
