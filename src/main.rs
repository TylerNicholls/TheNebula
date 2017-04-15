extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate rand;
extern crate time;
use std::env;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use std::f64;
//use rand::Rng;
use rand::distributions::{IndependentSample, Range};
//use std::num::Int;

const GREEN:    [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const YELLOW:   [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const RED:      [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const PURPLE:   [f32; 4] = [0.64, 0.0, 0.91, 1.0];
const GREY:     [f32; 4] = [0.1, 0.1, 0.1, 1.0];
const BLACK:    [f32; 4] = [0.0, 0.0, 0.0, 0.0];



//Window Size
const WINDOW_X: i32 = 800;
const WINDOW_Y: i32 = 600;

const PI :f64 = std::f64::consts::PI;
const DRAG :f64 = 0.250;  //simple drag

//struct containting information of the Player
pub struct Player {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    mass: f64,
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    up_d: bool, down_d: bool, left_d: bool, right_d: bool,
    score: i64,
    lives: i32
}

pub struct Enemy {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    mass: f64,
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    health: f64,
    theta: f64,
    bullet_theta: f64,
    damage_rate: f64
}

pub struct Nebula {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    x_pos: f64,
    y_pos: f64,
}

pub struct Bullet {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    x_pos: f64,
    y_pos: f64,
    x_vel: f64,
    y_vel: f64,
    exists: bool
}

pub struct Lives {
    gl: GlGraphics, // OpenGL drawing backend.
    radius: f64,
    x_pos: f64,
    y_pos: f64,
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

            let transform = c.transform.trans(x, y) //move reference to center of square
                                       .trans(-radius, -radius)
                                       .trans(x_pos, y_pos);

            ellipse(GREEN, shape1, transform, gl);
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
        if self.x_pos <= (-((WINDOW_X/2) as f64)+self.radius) {
            self.x_vel = -(self.x_vel);
            self.x_pos = -((WINDOW_X/2) as f64)+self.radius;
        } else if self.x_pos >= (((WINDOW_X/2) as f64)-self.radius) {
            self.x_vel = -(self.x_vel);
            self.x_pos = ((WINDOW_X/2) as f64)-self.radius;
        }
        if self.y_pos <= (-((WINDOW_Y/2) as f64)+self.radius) {
            self.y_vel = -(self.y_vel);
            self.y_pos = -((WINDOW_Y/2) as f64)+self.radius;
        } else if self.y_pos >= (((WINDOW_Y/2) as f64)-self.radius) {
            self.y_vel = -(self.y_vel);
            self.y_pos =((WINDOW_Y/2) as f64)-self.radius;
        }

        //Movement
        self.x_pos += self.x_vel * args.dt;
        self.y_pos += self.y_vel * args.dt;
        //Drag
        self.x_vel += -( (self.x_vel) * DRAG ) * args.dt ;
        self.y_vel += -( (self.y_vel) * DRAG ) * args.dt ;

    }
}

//implementation of the Enemy struct
impl Enemy {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let radius = self.radius;
        let shape1 = rectangle::square(0.0, 0.0, 2.0*self.radius);

        //let shape2 = rectangle::square( (2.0*self.radius - self.radius/3.0), (self.radius - self.radius/6.0), self.radius/3.0);

        let x_pos = self.x_pos; //TW
        let y_pos = self.y_pos; //TW

        let (x, y) = ((WINDOW_X / 2) as f64,
                      (WINDOW_Y / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {

            let transform = c.transform.trans(x, y) //move reference to center of shape
                                       .trans(-radius, -radius)
                                       .trans(x_pos, y_pos);

            ellipse(YELLOW, shape1, transform, gl);
            //ellipse(RED, shape2, transform, gl);
        } );



    }
    fn update(&mut self, args: &UpdateArgs) {
        //let vel_bump: f64  = 20.0;

        //update theta
        self.theta = self.theta + args.dt*PI/8.0;

        //boundaries
        if self.x_pos <= (-((WINDOW_X/2) as f64)+self.radius) {
            self.x_vel = -(self.x_vel);
            self.x_pos = -((WINDOW_X/2) as f64)+self.radius;
        } else if self.x_pos >= (((WINDOW_X/2) as f64)-self.radius) {
            self.x_vel = -(self.x_vel);
            self.x_pos = ((WINDOW_X/2) as f64)-self.radius;
        }
        if self.y_pos <= (-((WINDOW_Y/2) as f64)+self.radius) {
            self.y_vel = -(self.y_vel);
            self.y_pos = -((WINDOW_Y/2) as f64)+self.radius;
        } else if self.y_pos >= (((WINDOW_Y/2) as f64)-self.radius) {
            self.y_vel = -(self.y_vel);
            self.y_pos =((WINDOW_Y/2) as f64)-self.radius;
        }

        //Movement
        self.x_pos += self.x_vel * args.dt;
        self.y_pos += self.y_vel * args.dt;
        self.theta += 2.0 * args.dt;
        //Drag
        self.x_vel += -( (self.x_vel) * DRAG ) * args.dt ;
        self.y_vel += -( (self.y_vel) * DRAG ) * args.dt ;

        //Damage to enemy from being in the nebula
        self.health -= self.damage_rate * args.dt;
        //println!("x_vel: {:.2}, y_vel: {:.2}",self.x_vel,self.y_vel);
    }
}

//implementation of the Nebula struct
impl Nebula {
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
            clear(GREY, gl);

              let transform = c.transform.trans(x, y) //move reference to center of shape
                                        .trans(-radius, -radius)
                                        .trans(x_pos, y_pos);

               ellipse(PURPLE, shape1, transform, gl);
        });
      }
  }

//implementation of the Lives struct
impl Lives {
    fn render (&mut self, args: &RenderArgs) {
        use graphics::*;

        let radius = self.radius;
        let shape1 = rectangle::square(0.0, 0.0, 2.0*self.radius);
        let x_pos = self.x_pos;
        let y_pos = self.y_pos;

        let (x, y) = ((WINDOW_X / 2) as f64,
                      (WINDOW_Y / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {

            let transform = c.transform.trans(x, y) //move reference to center of shape
                .trans(-radius, -radius)
                .trans(x_pos, y_pos);

            ellipse(GREEN, shape1, transform, gl);
        });
    }
}



  impl Bullet {
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

                let transform = c.transform.trans(x, y) //move reference to center of shape
                                          .trans(-radius, -radius)
                                          .trans(x_pos, y_pos);

                 ellipse(RED, shape1, transform, gl);
          });
        }
    }


fn main() {
    let opengl = OpenGL::V3_2;
    let mut difficulty: i64 = 1;
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("The first argument is {}", args[1]);
        difficulty = args[1];
    }

    let start_time = time::get_time().sec;
    let mut window: Window = WindowSettings::new(
        "The Nebula!!",
        [(WINDOW_X as u32), (WINDOW_Y as u32)]
    )
    .opengl(opengl)
    .exit_on_esc(true)
    .build()
    .unwrap();

    //Random Number Generation
    //let between_x = Range::new(-((WINDOW_X/2) as f64), ((WINDOW_X/2) as f64));
    //let between_y = Range::new(-((WINDOW_Y/2) as f64), ((WINDOW_Y/2) as f64));
    //let mut rng = rand::thread_rng();
    //println!("rando: ({}, {})", between_x.ind_sample(&mut rng), between_y.ind_sample(&mut rng));

    //time
    //let start_time = time::now();


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
     right_d: false,
      score: 0,
     lives: 3
 };

 let mut enemy = Enemy {
     gl: GlGraphics::new(opengl),
     radius: 37.5,
     mass: 1.0,
     x_pos: 90.0,
     y_pos: 90.0,
     x_vel: 0.0,
     y_vel: 0.0,
     health: 100.0,
     theta: 0.0,
     bullet_theta: 0.0,
     damage_rate: 0.0
 };

 let mut nebula = Nebula {
     gl: GlGraphics::new(opengl),
     radius: 75.0,
     x_pos: -90.0,
     y_pos: -90.0
 };

 let mut life1 = Lives {
     gl: GlGraphics::new(opengl),
     radius: 10.0,
     x_pos: -370.0,
     y_pos: -270.0
    };

 let mut life2 = Lives {
        gl: GlGraphics::new(opengl),
        radius: 10.0,
        x_pos: -345.0,
        y_pos: -270.0
    };

 let mut life3 = Lives {
        gl: GlGraphics::new(opengl),
        radius: 10.0,
        x_pos: -320.0,
        y_pos: -270.0
    };

 let mut bullet = Bullet {
     gl: GlGraphics::new(opengl),
     radius: 5.0,
     x_pos: enemy.x_pos,
     y_pos: enemy.y_pos,
     x_vel: 50.0,
     y_vel: 50.0,
     exists: false
 };


 let mut can_shoot: bool = true;
 let mut is_start: bool = true;
 //let mut bullet_exists: bool = false;

//the event loop
 let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {

        let current_time = time::get_time();


        println!("start_time: {}", current_time.sec );
        if (is_start || current_time.sec%3 == 0) && (can_shoot) {
            println!("Shoot! start_time: {}", current_time.sec );
            can_shoot = false;
            is_start = false;
            bullet.exists = true;
        } else if current_time.sec%3 != 0 {
            can_shoot = true;
        }

        if let Some(r) = e.render_args() {
            nebula.render(&r);
            player.render(&r);
            enemy.render(&r);
            bullet.render(&r);
            life1.render(&r);

            if player.lives > 1 {
                life2.render(&r);
                if player.lives > 2 {
                    life3.render(&r);
                }
            }

         }


        if let Some(u) = e.update_args() {
            let mut wait: i64 = 0;
            if wait - time::get_time().sec < 0{
                wait = 0;
            }
            player.update(&u);
            enemy.update(&u);
            //let returner: bool = collision(&mut player, &mut enemy);
            if collision(&mut player, &mut enemy) {
                rebound(&mut player, &mut enemy);
                //println!("collide");
            }
            damage_enemy(&mut enemy, &mut nebula);
            if bullet.exists && wait == 0 {
                update_bullet(&mut bullet, &mut enemy, difficulty);
            }else {
                reset_bullet(&mut bullet, &mut enemy);
            }

            if bullet_collision(&mut bullet, &mut player, &mut enemy) {
                println!("hit!! Lives: {}",player.lives);
                wait = time::get_time().sec;
                if player.lives <= 0 {
                    lose(&mut player, &mut enemy, &mut bullet, start_time);
                }
            }
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
        }   //end button

    }   //end while

}   //end main

fn reset_bullet(the_bullet: &mut Bullet, the_enemy: &mut Enemy){
    the_bullet.x_pos = the_enemy.x_pos;
    the_bullet.y_pos = the_enemy.y_pos;
    the_enemy.bullet_theta = the_enemy.theta;
}

fn update_bullet(the_bullet: &mut Bullet, the_enemy: &mut Enemy, difficulty: i64){
    if the_bullet.x_pos <= (-((WINDOW_X/2) as f64)+the_bullet.radius) {
        reset_bullet(the_bullet, the_enemy);

    } else if the_bullet.x_pos >= (((WINDOW_X/2) as f64)-the_bullet.radius) {
        reset_bullet(the_bullet, the_enemy);
    }
    if the_bullet.y_pos <= (-((WINDOW_Y/2) as f64)+the_bullet.radius) {
        reset_bullet(the_bullet, the_enemy);

    } else if the_bullet.y_pos >= (((WINDOW_Y/2) as f64)-the_bullet.radius) {
        reset_bullet(the_bullet, the_enemy);
    }


    let bullet_speed: f64 = 5.0 * difficulty as f64;
    the_bullet.x_vel = bullet_speed * the_enemy.bullet_theta.cos();
    the_bullet.y_vel = bullet_speed * the_enemy.bullet_theta.sin();
    the_bullet.x_pos += the_bullet.x_vel;
    the_bullet.y_pos += the_bullet.y_vel;

}
fn win(the_player: &mut Player, the_enemy: &mut Enemy, the_bullet: &mut Bullet, start_time: i64){
    println!("Enemy is Dead.");
    let win_bonus: i64 = 100000;
    the_player.score += (1000 - (start_time - time::get_time().sec)) *1000 + win_bonus;
    println!("score {:.2}", the_player.score);

    reset_frame(the_player, the_enemy, the_bullet);
}

fn lose(the_player: &mut Player, the_enemy: &mut Enemy, the_bullet: &mut Bullet, start_time: i64){
    println!("Player is Dead.");
    the_player.score += (1000 - (start_time - time::get_time().sec)) *1000;
    println!("score {:.2}", the_player.score);

    reset_frame(the_player, the_enemy, the_bullet);
}

fn reset_frame(the_player: &mut Player, the_enemy: &mut Enemy, the_bullet: &mut Bullet){
        println!("start");
        while true {
            //clear(GREY, the_player.gl);
        }
}


//boundaries
fn collision(the_player: &mut Player, the_enemy: &mut Enemy) -> bool {
    let mut retval: bool = false;
    the_player.score += (((the_player.x_vel.powi(2) + the_player.y_vel.powi(2)).sqrt()) as i64) * 100 ;
    let cent_dist: f64 = ((the_player.x_pos-the_enemy.x_pos).powi(2) + (the_player.y_pos-the_enemy.y_pos).powi(2) ).sqrt();
    //println!("centDist: {:.2}",cent_dist);
    if cent_dist <= (the_enemy.radius + the_player.radius) {
        retval = true;
    }
    retval
}

fn rebound(the_player: &mut Player, the_enemy: &mut Enemy) {

        let player_theta:f64 = (the_player.y_vel).atan2(the_player.x_vel);  //vector angle of player velocity
        let enemy_theta:f64 = (the_enemy.y_vel).atan2(the_enemy.x_vel);     //vector angle of enemy velocity
        let phi:f64 = (the_enemy.y_pos - the_player.y_pos).atan2(the_enemy.x_pos - the_player.x_pos);   //TODO: Check this
        let enemy_net_vel: f64  = ( the_enemy.x_vel.powi(2)  + the_enemy.y_vel.powi(2)  ).sqrt();
        let player_net_vel: f64 = ( the_player.x_vel.powi(2) + the_player.y_vel.powi(2) ).sqrt();

        the_enemy.x_vel  = (enemy_net_vel *  (enemy_theta - phi).cos()  * (the_enemy.mass - the_player.mass) + 2.0* the_player.mass*player_net_vel*(player_theta - phi).cos() * (phi).cos())/(the_enemy.mass + the_player.mass) + enemy_net_vel  * (enemy_theta - phi).sin()  * (phi + PI/2.0).cos();
        the_enemy.y_vel  = (enemy_net_vel *  (enemy_theta - phi).cos()  * (the_enemy.mass - the_player.mass) + 2.0* the_player.mass*player_net_vel*(player_theta - phi).cos() * (phi).sin())/(the_enemy.mass + the_player.mass) + enemy_net_vel  * (enemy_theta - phi).sin()  * (phi + PI/2.0).sin();

        the_player.x_vel = (player_net_vel * (player_theta - phi).cos() * (the_player.mass - the_enemy.mass) + 2.0* the_enemy.mass*enemy_net_vel*(enemy_theta - phi).cos()  *   (phi).cos())/(the_player.mass + the_enemy.mass) + player_net_vel * (player_theta - phi).sin() * (phi + PI/2.0).cos();
        the_player.y_vel = (player_net_vel * (player_theta - phi).cos() * (the_player.mass - the_enemy.mass) + 2.0* the_enemy.mass*enemy_net_vel*(enemy_theta - phi).cos()  *   (phi).sin())/(the_player.mass + the_enemy.mass) + player_net_vel * (player_theta - phi).sin() * (phi + PI/2.0).sin();
}

fn damage_enemy(the_enemy: &mut Enemy, the_nebula: &mut Nebula) {
    let damage_factor: f64 = 2.0;
    let cent_dist: f64 = ((the_nebula.x_pos-the_enemy.x_pos).powi(2) + (the_nebula.y_pos-the_enemy.y_pos).powi(2) ).sqrt();
    if cent_dist <= (the_enemy.radius + the_nebula.radius)  {
        if (cent_dist + the_enemy.radius) <= the_nebula.radius {    //completely in the nebula
            the_enemy.damage_rate = damage_factor;
        } else {        //partially in the nebula
            let first_part: f64  = the_enemy.radius.powi(2)  * ( (cent_dist.powi(2)+ the_enemy.radius.powi(2)  - the_nebula.radius.powi(2)) / (2.0*cent_dist*the_enemy.radius)).acos();
            let second_part: f64 = the_nebula.radius.powi(2) * ( (cent_dist.powi(2)+ the_nebula.radius.powi(2) - the_enemy.radius.powi(2)) / (2.0*cent_dist*the_nebula.radius)).acos();
            let third_part: f64  = -0.5* ( (-cent_dist + the_enemy.radius + the_nebula.radius) * (cent_dist + the_enemy.radius - the_nebula.radius) * (cent_dist - the_enemy.radius + the_nebula.radius) * (cent_dist + the_enemy.radius + the_nebula.radius) ).sqrt();
            the_enemy.damage_rate = damage_factor * (first_part + second_part + third_part) / (PI*the_enemy.radius.powi(2));
        }
        println!("damage! Health = {:.2}",the_enemy.health);
    }
}

fn bullet_collision(the_bullet: &mut Bullet, the_player: &mut Player, the_enemy: &mut Enemy) -> bool {
    let mut retval: bool = false;
    let cent_dist: f64 = ((the_player.x_pos-the_bullet.x_pos).powi(2) + (the_player.y_pos-the_bullet.y_pos).powi(2) ).sqrt();
    //println!("centDist: {:.2}",cent_dist);
    if cent_dist <= (the_bullet.radius + the_player.radius) {
        retval = true;
        if the_bullet.exists {
            the_player.lives -= 1;
            reset_bullet(the_bullet, the_enemy);
            the_bullet.exists = false;
        }
    }
    retval
}
