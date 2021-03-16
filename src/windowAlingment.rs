#[allow(non_snake_case)]

pub enum WINDOWALINGMENT {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

pub struct WINDOWPOS {
    screen_width: i32,
    screen_height: i32,
    offsetFromBorders: i32,
    percentileOffsetHeight: f32,
    percentileOffsetWidth: f32,
    alingment: WINDOWALINGMENT,
}

impl WINDOWPOS {
    pub fn new(
        (w, h): (i32, i32),
        borderOffset: i32,
        offsetWidth: f32,
        offsetHeight: f32,
        aling: WINDOWALINGMENT,
    ) -> Self {
        return Self {
            alingment: aling,
            screen_height: h,
            screen_width: w,
            offsetFromBorders: borderOffset,
            percentileOffsetHeight: offsetHeight,
            percentileOffsetWidth: offsetWidth,
        };
    }
    pub fn getWindowPos(&self) -> (i32, i32) {
        match self.alingment {
            WINDOWALINGMENT::BottomRight => {
                return (
                    self.screen_width
                        - self.percentFromVal(self.screen_width as f32, self.percentileOffsetWidth)
                            as i32
                        - self.offsetFromBorders,
                    self.screen_height
                        - self
                            .percentFromVal(self.screen_height as f32, self.percentileOffsetHeight)
                            as i32
                        - self.offsetFromBorders,
                )
            }
            WINDOWALINGMENT::BottomLeft => {
                return (
                    self.offsetFromBorders,
                    self.screen_height
                        - self
                            .percentFromVal(self.screen_height as f32, self.percentileOffsetHeight)
                            as i32
                        - self.offsetFromBorders,
                )
            }
            WINDOWALINGMENT::TopLeft => return (self.offsetFromBorders, self.offsetFromBorders),
            WINDOWALINGMENT::TopRight => {
                return (
                    self.screen_width
                        - self.percentFromVal(self.screen_width as f32, self.percentileOffsetWidth)
                            as i32
                        - self.offsetFromBorders,
                    self.offsetFromBorders,
                )
            }
        }
    }

    pub fn getSize(&self) -> (i32, i32) {
        return (
            self.percentFromVal(self.screen_width as f32, self.percentileOffsetWidth) as i32,
            self.percentFromVal(self.screen_height as f32, self.percentileOffsetHeight) as i32,
        );
    }

    fn percentFromVal(&self, val: f32, percent: f32) -> f32 {
        val * (percent * 0.01_f32)
    }
}
