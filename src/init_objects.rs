use macroquad::prelude::*;
use crate::helper::*;

pub struct Objects{
    pub ball: MyCircle,
    pub bar1: MyRectangle,
    pub bar2: MyRectangle,
    pub midfield: MyRectangle,
    pub v: Velocity,
}

pub fn init_objects() -> Objects {
    let ball =MyCircle{
        x: screen_width()/2.0,
        y: screen_height()/2.0,
        r: 15.0,
        clr: WHITE
    };

    let bar1 = MyRectangle{
        x: -5.0, // temporary fix for top collision
        y: screen_height()/2.0,
        w: 15.0,
        h: 150.0,
        clr: WHITE 
    };

    let bar2 = MyRectangle{
        x: screen_width() - bar1.w + 5.0,
        y: screen_height()/2.0,
        w: 15.0,
        h: 150.0,
        clr: WHITE 
    };

    let midfield = MyRectangle{
        x: screen_width()/2.0,
        y: 0.0,
        w: 1.0,
        h: screen_height(),
        clr: WHITE
    };

    let v = Velocity{
        x: -5.0,
        y: 0.0 
    };

    Objects {
            ball,
            bar1,
            bar2,
            midfield,
            v,
    }
}     