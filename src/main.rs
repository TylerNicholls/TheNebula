extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::f64;
//use std::num::Int;

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const PURPLE:   [f32; 4] = [0.64, 0.0, 0.91, 1.0];
const GREY:     [f32; 4] = [0.1, 0.1, 0.1, 1.0];

const WINDOW_X: i32 = 600;
const WINDOW_Y: i32 = 600;

const PI :f64 = std::f64::consts::PI;

//struct containting information of the Player
pub struct Player {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    mass: f64,
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool
}

pub struct Enemy {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    mass: f64,
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
}

//implementation of the Player struct
impl Player {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let radius = self.radius;
        let shape1 = rectangle::square(0.0, 0.0, 2.0*self.radius);
        let x_pos = self.x_pos; //TW
        let y_pos = self.y_pos; //TW

        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREY, gl);

            let transform = c.transform.trans(x, y) //move reference to center of square
                                       .trans(-radius, -radius)
                                       .trans(x_pos, y_pos);

            // Draw a box rotating around the middle of the screen.
            //rectangle(PURPLE, square, transform, gl);
            ellipse(PURPLE, shape1, transform, gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
        let vel_bump: f64  = 20.0;
        if self.up_d {
            self.y_vel += -vel_bump;
            self.up_d = false;

        }
        if self.down_d {
            self.y_vel += vel_bump;
            self.down_d = false;

        }
        if self.left_d {
            self.x_vel += -vel_bump;
            self.left_d = false;
        }
        if self.right_d {
            self.x_vel += vel_bump;
            self.right_d = false;
        }
        //boundaries
        if (self.x_pos <= (-300.0+self.radius)) || (self.x_pos >= (300.0-self.radius)) {
            self.x_vel = -(self.x_vel);
        }
        if (self.y_pos <= (-300.0+self.radius)) || (self.y_pos >= (300.0-self.radius)) {
            self.y_vel = -(self.y_vel);
        }
        self.x_pos += self.x_vel * args.dt;
        self.y_pos += self.y_vel * args.dt;
        let drag: f64 = 0.250;  //simple drag
        //let mass: f64 = 1.0;    //mass

        self.x_vel += -( (self.x_vel) * drag ) * args.dt ;
        self.y_vel += -( (self.y_vel) * drag ) * args.dt ;
        //println!("x_vel: {:.2}, y_vel: {:.2}",self.x_vel,self.y_vel);
    }
}



//implementation of the Player struct
impl Enemy {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let radius = self.radius;
        let shape1 = rectangle::square(0.0, 0.0, 2.0*self.radius);
        let x_pos = self.x_pos; //TW
        let y_pos = self.y_pos; //TW

        let (x, y) = ((WINDOW_X / 2) as f64,
                      (WINDOW_Y / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //clear(GREY, gl);

            let transform = c.transform.trans(x, y) //move reference to center of square
                                       .trans(-radius, -radius)
                                       .trans(x_pos, y_pos);

            // Draw a box rotating around the middle of the screen.
            //rectangle(PURPLE, square, transform, gl);
            ellipse(GREEN, shape1, transform, gl);
        });
    }
    fn update(&mut self, args: &UpdateArgs) {
        //let vel_bump: f64  = 20.0;

        //boundaries
        if (self.x_pos <= (-300.0+self.radius)) || (self.x_pos >= (300.0-self.radius)) {
            self.x_vel = -(self.x_vel);
        }
        if (self.y_pos <= (-300.0+self.radius)) || (self.y_pos >= (300.0-self.radius)) {
            self.y_vel = -(self.y_vel);
        }
        self.x_pos += self.x_vel * args.dt;
        self.y_pos += self.y_vel * args.dt;
        let drag: f64 = 0.250;  //simple drag
        //let mass: f64 = 1.0;    //mass

        self.x_vel += -( (self.x_vel) * drag ) * args.dt ;
        self.y_vel += -( (self.y_vel) * drag ) * args.dt ;
        //println!("x_vel: {:.2}, y_vel: {:.2}",self.x_vel,self.y_vel);

    }
}






fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "The Nebula!!",
        [600, 600]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    // Create a new game and run it.
 let mut player = Player {
     gl: GlGraphics::new(opengl),
     radius: 50.0,
     mass: 1.0,
     x_pos: 0.0,
     y_pos: 0.0,
     x_vel: 0.0,
     y_vel: 0.0,
     up_d: false,
     down_d: false,
     left_d: false,
     right_d: false
 };

 let mut enemy = Enemy {
     gl: GlGraphics::new(opengl),
     radius: 37.5,
     mass: 1.0,
     x_pos: 90.0,
     y_pos: 90.0,
     x_vel: 0.0,
     y_vel: 0.0,
 };

//collision(player, enemy);

//the event loop
 let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            player.render(&r);
            enemy.render(&r);
        }

        if let Some(u) = e.update_args() {
            player.update(&u);
            enemy.update(&u);
            //let returner: bool = collision(&mut player, &mut enemy);
            if collision(&mut player, &mut enemy) {
                rebound(&mut player, &mut enemy);
                println!("collide");
            }
            //collision(&mut player, &mut enemy);
        }

        if let Some(button) = e.press_args() {
            use Button::Keyboard;
            use Key;

            if button == Keyboard(Key::Up) {
                //println!("Up button pressed");
                player.up_d = true;
            }
            if button == Keyboard(Key::Down) {
                //println!("Down button pressed");
                player.down_d = true;
            }
            if button == Keyboard(Key::Left) {
                //println!("Left button pressed");
                player.left_d = true;
            }
            if button == Keyboard(Key::Right) {
                //println!("Right button pressed");
                player.right_d = true;
            }

        }
    }   //end while

}   //end main


fn collision(the_player: &mut Player, the_enemy: &mut Enemy) -> bool {
    let mut retval: bool = false;
    //println!("the_player.x_pos: {:.2}, the_enemy.y_pos: {:.2}",the_player.x_pos, the_enemy.x_pos);
    let cent_dist: f64 = ((the_player.x_pos-the_enemy.x_pos).powi(2) + (the_player.y_pos-the_enemy.y_pos).powi(2) ).sqrt();
    println!("centDist: {:.2}",cent_dist);
    if cent_dist <= (the_enemy.radius + the_player.radius) {
        retval = true;
    }
    retval
}

fn rebound(the_player: &mut Player, the_enemy: &mut Enemy) {

        let player_theta:f64 = (the_player.x_vel).atan2(the_player.y_vel);
        let enemy_theta:f64 = (the_enemy.x_vel).atan2(the_enemy.y_vel);
        let phi:f64 = (the_enemy.y_pos - the_player.y_pos).atan2(the_enemy.x_pos - the_enemy.x_pos);
        let enemy_net_vel: f64 = ( the_enemy.x_vel.powi(2) + the_enemy.y_vel.powi(2) ).sqrt();
        let player_net_vel: f64 = ( the_player.x_vel.powi(2) + the_player.y_vel.powi(2) ).sqrt();

        the_enemy.x_vel  = (enemy_net_vel * (enemy_theta - phi).cos() * (the_enemy.mass - the_player.mass) + 2.0* the_player.mass*player_net_vel*(player_theta - phi).cos()*(phi).cos())/(the_enemy.mass + the_player.mass) + enemy_net_vel * (enemy_theta - phi).sin() * (phi + PI/2.0).cos();
        the_enemy.y_vel  = (enemy_net_vel * (enemy_theta - phi).cos() * (the_enemy.mass - the_player.mass) + 2.0* the_player.mass*player_net_vel*(player_theta - phi).cos()*(phi).sin())/(the_enemy.mass + the_player.mass) + enemy_net_vel * (enemy_theta - phi).sin() * (phi + PI/2.0).sin();

        the_player.x_vel  = (player_net_vel * (player_theta - phi).cos() * (the_player.mass - the_enemy.mass) + 2.0* the_enemy.mass*enemy_net_vel*(enemy_theta - phi).cos()*(phi).cos())/(the_player.mass + the_enemy.mass) + player_net_vel * (player_theta - phi).sin() * (phi + PI/2.0).cos();
        the_player.y_vel  = (player_net_vel * (player_theta - phi).cos() * (the_player.mass - the_enemy.mass) + 2.0* the_enemy.mass*enemy_net_vel*(enemy_theta - phi).cos()*(phi).sin())/(the_player.mass + the_enemy.mass) + player_net_vel * (player_theta - phi).sin() * (phi + PI/2.0).sin();

}
