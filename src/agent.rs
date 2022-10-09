use std::io::ErrorKind;
use crate::map::Map;

pub struct Agent {
    xpos: u32,
    ypos: u32,
    angle: f64, //0 angle is east, positive anti-clockwise
    velocity: u32
}

impl Agent {
    pub fn update<const WIDTH: usize, const HEIGHT: usize, const NUMBER_OF_AGENTS: usize>(&mut self, map: Map<WIDTH, HEIGHT, NUMBER_OF_AGENTS>) -> Result<(u32, u32), ErrorKind> {
        const POLL_DISTANCE: f64 = 1.1;
        const SIDE_ANGLE_ADJUST: f64 = 0.3;

        let poll_left = map.poll(self.xpos, self.ypos, self.angle + SIDE_ANGLE_ADJUST, POLL_DISTANCE);
        let poll_front = map.poll(self.xpos, self.ypos, self.angle, POLL_DISTANCE);
        let poll_right = map.poll(self.xpos, self.ypos, self.angle - SIDE_ANGLE_ADJUST, POLL_DISTANCE);

        if poll_left > poll_front && poll_left > poll_right {
            self.xpos = self.xpos + self.velocity * f64::sin(self.angle + SIDE_ANGLE_ADJUST) as u32;
            self.ypos = self.ypos + self.velocity * f64::cos(self.angle + SIDE_ANGLE_ADJUST) as u32;
        } else if poll_right > poll_front && poll_right > poll_left {
            self.xpos = self.xpos + self.velocity * f64::sin(self.angle - SIDE_ANGLE_ADJUST) as u32;
            self.ypos = self.ypos + self.velocity * f64::cos(self.angle - SIDE_ANGLE_ADJUST) as u32;
        } else {
            self.xpos = self.xpos + self.velocity * f64::sin(self.angle) as u32;
            self.ypos = self.ypos + self.velocity * f64::cos(self.angle) as u32;
        }
    }
}