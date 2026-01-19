pub type ScoreMap = phf::Map<&'static str, i16>;

pub struct Model {
    pub total_score: i32,
    pub uw1: &'static ScoreMap,
    pub uw2: &'static ScoreMap,
    pub uw3: &'static ScoreMap,
    pub uw4: &'static ScoreMap,
    pub uw5: &'static ScoreMap,
    pub uw6: &'static ScoreMap,
    pub bw1: &'static ScoreMap,
    pub bw2: &'static ScoreMap,
    pub bw3: &'static ScoreMap,
    pub tw1: &'static ScoreMap,
    pub tw2: &'static ScoreMap,
    pub tw3: &'static ScoreMap,
    pub tw4: &'static ScoreMap,
}

impl Model {
    pub fn total_score(&self) -> i32 {
        self.total_score
    }
}
