use conifer::prelude::*;

use env_logger::init;
use log::{debug, info, warn};

fn main() {
    env_logger::init();
    info!("Starting");
    let mut d = Config::auto().unwrap();

    d.run(|frame, swipe, delta_time| {
        debug!("Enter callback");
        if let Some(swipe) = swipe {
            debug!("New swipe");
            let points = swipe.points.clone();
            if points.iter().any(|p| p.x > 750) {
                // exit if we touch right of the screen
                return Ok(RunResponse::Exit);
            }
            debug!("{:?}", swipe.drag());
            if let Some(Gesture::Drag(point0, point1)) = swipe.drag() {
                debug!("Draw line");
                frame.plotLine(point0, point1);
            }
        }
        Ok(RunResponse::Draw)
    })
}