use sb_sbity::target::Stage as StageData;

pub struct Stage {
    stage: StageData,
}

impl Stage {
    pub fn new(stage: StageData) -> Self {
        Self { stage }
    }

    pub fn flush(&mut self) {
        println!("Stage flush");
    }
}
