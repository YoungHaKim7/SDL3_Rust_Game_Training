use sdl3::pixels::Color;
use sdl3::rect::Point;
use sdl3::render::Canvas;
use sdl3::video::Window;

use std::f32::consts::PI;

use crate::DEBUG_STRING;

pub const MAP_BOX_SCALE: i32 = 16;
pub const MAP_BOX_EDGES_LEN: usize = 12 + (MAP_BOX_SCALE * 2) as usize;
pub const MAX_PLAYER_COUNT: usize = 4;
pub const CIRCLE_DRAW_SIDES: usize = 32;
pub const CIRCLE_DRAW_SIDES_LEN: usize = CIRCLE_DRAW_SIDES + 1;

#[derive(Default, Clone, Copy)]

pub struct Player {
    pub mouse: u32,
    pub keyboard: u32,
    pub pos: [f32; 3],
    pub vel: [f32; 3],
    pub yaw: u32,
    pub pitch: i32,
    pub radius: f32,
    pub height: f32,
    pub color: [u8; 3],
    pub wasd: u8,
}

pub fn whose_mouse(mouse: u32, players: &[Player]) -> Option<usize> {
    players.iter().position(|player| player.mouse == mouse)
}

pub fn whose_keyboard(keyboard: u32, players: &[Player]) -> Option<usize> {
    players
        .iter()
        .position(|player| player.keyboard == keyboard)
}

pub fn shoot(shooter: usize, players: &mut [Player]) {
    let x0 = players[shooter].pos[0];
    let y0 = players[shooter].pos[1];
    let z0 = players[shooter].pos[2];
    let bin_rad = PI / 2147483648.0;
    let yaw_rad = bin_rad * players[shooter].yaw as f32;
    let pitch_rad = bin_rad * players[shooter].pitch as f32;
    let cos_yaw = yaw_rad.cos();
    let sin_yaw = yaw_rad.sin();
    let cos_pitch = pitch_rad.cos();
    let sin_pitch = pitch_rad.sin();
    let vx = -sin_yaw * cos_pitch;
    let vy = sin_pitch;
    let vz = -cos_yaw * cos_pitch;

    for (i, target) in players.iter_mut().enumerate() {
        if i == shooter {
            continue;
        }
        let mut hit = 0;
        for j in 0..2 {
            let r = target.radius as f32;
            let h = target.height as f32;
            let dx = target.pos[0] - x0;
            let dy = target.pos[1] - y0 + if j == 0 { 0.0 } else { r - h };
            let dz = target.pos[2] - z0;
            let vd = vx * dx + vy * dy + vz * dz;
            let dd = dx * dx + dy * dy + dz * dz;
            let vv = vx * vx + vy * vy + vz * vz;
            let rr = r * r;
            if vd < 0.0 {
                continue;
            }
            if vd * vd >= vv * (dd - rr) {
                hit += 1;
            }
        }
        if hit > 0 {
            target.pos[0] = (MAP_BOX_SCALE * (rand::random::<i32>() % 256 - 128)) as f32 / 256.0;
            target.pos[1] = (MAP_BOX_SCALE * (rand::random::<i32>() % 256 - 128)) as f32 / 256.0;
            target.pos[2] = (MAP_BOX_SCALE * (rand::random::<i32>() % 256 - 128)) as f32 / 256.0;
        }
    }
}

pub fn update(players: &mut [Player], players_len: usize, dt_ns: u64) {
    let rate = 6.0;
    let time = dt_ns as f32 * 1e-9;
    let drag = (-time * rate).exp();
    let diff = 1.0 - drag;
    let mult = 60.0;
    let grav = 25.0;

    for player in players.iter_mut().take(players_len) {
        let yaw = player.yaw as f32;
        let rad = yaw * PI / 2147483648.0;
        let cos = rad.cos();
        let sin = rad.sin();
        let wasd = player.wasd;
        let dir_x = if wasd & 8 != 0 { 1.0 } else { 0.0 } - if wasd & 2 != 0 { 1.0 } else { 0.0 };
        let dir_z = if wasd & 4 != 0 { 1.0 } else { 0.0 } - if wasd & 1 != 0 { 1.0 } else { 0.0 };
        let norm = (dir_x * dir_x + dir_z * dir_z).sqrt() as f32;
        let acc_x = mult
            * if norm == 0.0 {
                0.0
            } else {
                (cos * dir_x + sin * dir_z) / norm
            };
        let acc_z = mult
            * if norm == 0.0 {
                0.0
            } else {
                (-sin * dir_x + cos * dir_z) / norm
            };

        player.vel[0] -= player.vel[0] * diff;
        player.vel[1] -= grav * time;
        player.vel[2] -= player.vel[2] * diff;
        player.vel[0] += diff * acc_x / rate;
        player.vel[2] += diff * acc_z / rate;
        player.pos[0] += (time - diff / rate) * acc_x / rate + diff * player.vel[0] / rate;
        player.pos[1] += -0.5 * grav * time * time + player.vel[1] * time;
        player.pos[2] += (time - diff / rate) * acc_z / rate + diff * player.vel[2] / rate;

        let scale = MAP_BOX_SCALE as f32;
        let bound = scale - player.radius as f32;
        let pos_x = player.pos[0].min(bound).max(-bound);
        let pos_y = player.pos[1].min(bound).max(player.height as f32 - scale);
        let pos_z = player.pos[2].min(bound).max(-bound);

        if player.pos[0] != pos_x {
            player.vel[0] = 0.0;
        }
        if player.pos[1] != pos_y {
            player.vel[1] = if wasd & 16 != 0 { 8.4375 } else { 0.0 };
        }
        if player.pos[2] != pos_z {
            player.vel[2] = 0.0;
        }

        player.pos[0] = pos_x;
        player.pos[1] = pos_y;
        player.pos[2] = pos_z;
    }
}

pub fn draw_circle(canvas: &mut Canvas<Window>, r: f32, x: f32, y: f32) {
    let mut points = Vec::with_capacity(CIRCLE_DRAW_SIDES_LEN);
    for i in 0..CIRCLE_DRAW_SIDES_LEN {
        let ang = 2.0 * PI as f32 * i as f32 / CIRCLE_DRAW_SIDES as f32;
        points.push(Point::new(
            (x + r * ang.cos()) as i32,
            (y + r * ang.sin()) as i32,
        ));
    }
    canvas.draw_lines(&points).unwrap();
}

pub fn draw_clipped_segment(
    canvas: &mut Canvas<Window>,
    ax: f32,
    ay: f32,
    az: f32,
    bx: f32,
    by: f32,
    bz: f32,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    if az >= -w && bz >= -w {
        return;
    }

    let mut ax = ax;
    let mut ay = ay;
    let mut az = az;
    let mut bx = bx;
    let mut by = by;
    let mut bz = bz;

    let dx = ax - bx;
    let dy = ay - by;

    if az > -w {
        let t = (-w - bz) / (az - bz);
        ax = bx + dx * t;
        ay = by + dy * t;
        az = -w;
    } else if bz > -w {
        let t = (-w - az) / (bz - az);
        bx = ax - dx * t;
        by = ay - dy * t;
        bz = -w;
    }

    ax = -z * ax / az;
    ay = -z * ay / az;
    bx = -z * bx / bz;
    by = -z * by / bz;

    canvas
        .draw_line(
            Point::new((x + ax) as i32, (y - ay) as i32),
            Point::new((x + bx) as i32, (y - by) as i32),
        )
        .unwrap();
}

pub fn draw(
    canvas: &mut Canvas<Window>,
    edges: &[[f32; 6]],
    players: &[Player],
    players_len: usize,
) {
    let (w, h) = canvas.output_size().unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    if players_len > 0 {
        let wf = w as f32;
        let hf = h as f32;
        let part_hor = if players_len > 2 { 2 } else { 1 };
        let part_ver = if players_len > 1 { 2 } else { 1 };
        let size_hor = wf / part_hor as f32;
        let size_ver = hf / part_ver as f32;

        for (i, player) in players.iter().enumerate() {
            let mod_x = (i % part_hor) as f32;
            let mod_y = (i / part_hor) as f32;
            let hor_origin = (mod_x + 0.5) * size_hor;
            let ver_origin = (mod_y + 0.5) * size_ver;
            let cam_origin = 0.5 * (size_hor * size_hor + size_ver * size_ver).sqrt();
            let hor_offset = mod_x * size_hor;
            let ver_offset = mod_y * size_ver;

            let rect = sdl3::rect::Rect::new(
                hor_offset as i32,
                ver_offset as i32,
                size_hor as u32,
                size_ver as u32,
            );
            canvas.set_clip_rect(rect);

            let x0 = player.pos[0];
            let y0 = player.pos[1];
            let z0 = player.pos[2];
            let bin_rad = PI / 2147483648.0;
            let yaw_rad = bin_rad * player.yaw as f32;
            let pitch_rad = bin_rad * player.pitch as f32;
            let cos_yaw = yaw_rad.cos();
            let sin_yaw = yaw_rad.sin();
            let cos_pitch = pitch_rad.cos();
            let sin_pitch = pitch_rad.sin();
            let mat = [
                cos_yaw,
                0.0,
                -sin_yaw,
                sin_yaw * sin_pitch,
                cos_pitch,
                cos_yaw * sin_pitch,
                sin_yaw * cos_pitch,
                -sin_pitch,
                cos_yaw * cos_pitch,
            ];

            canvas.set_draw_color(Color::RGB(64, 64, 64));
            for line in edges.iter() {
                let ax = (mat[0] * (line[0] as f32 - x0)
                    + mat[1] * (line[1] as f32 - y0)
                    + mat[2] * (line[2] as f32 - z0)) as f32;
                let ay = (mat[3] * (line[0] as f32 - x0)
                    + mat[4] * (line[1] as f32 - y0)
                    + mat[5] * (line[2] as f32 - z0)) as f32;
                let az = (mat[6] * (line[0] as f32 - x0)
                    + mat[7] * (line[1] as f32 - y0)
                    + mat[8] * (line[2] as f32 - z0)) as f32;
                let bx = (mat[0] * (line[3] as f32 - x0)
                    + mat[1] * (line[4] as f32 - y0)
                    + mat[2] * (line[5] as f32 - z0)) as f32;
                let by = (mat[3] * (line[3] as f32 - x0)
                    + mat[4] * (line[4] as f32 - y0)
                    + mat[5] * (line[5] as f32 - z0)) as f32;
                let bz = (mat[6] * (line[3] as f32 - x0)
                    + mat[7] * (line[4] as f32 - y0)
                    + mat[8] * (line[5] as f32 - z0)) as f32;
                draw_clipped_segment(
                    canvas,
                    ax,
                    ay,
                    az,
                    bx,
                    by,
                    bz,
                    hor_origin,
                    ver_origin,
                    cam_origin as f32,
                    1.0,
                );
            }

            for (j, target) in players.iter().enumerate() {
                if i == j {
                    continue;
                }
                canvas.set_draw_color(Color::RGB(
                    target.color[0],
                    target.color[1],
                    target.color[2],
                ));
                for k in 0..2 {
                    let rx = target.pos[0] - player.pos[0];
                    let ry = target.pos[1] - player.pos[1]
                        + (target.radius - target.height) as f32 * k as f32;
                    let rz = target.pos[2] - player.pos[2];
                    let dx = mat[0] * rx + mat[1] * ry + mat[2] * rz;
                    let dy = mat[3] * rx + mat[4] * ry + mat[5] * rz;
                    let dz = mat[6] * rx + mat[7] * ry + mat[8] * rz;
                    let r_eff = target.radius as f32 * cam_origin / dz;
                    if dz >= 0.0 {
                        continue;
                    }
                    draw_circle(
                        canvas,
                        r_eff as f32,
                        (hor_origin - cam_origin * dx / dz) as f32,
                        (ver_origin + cam_origin * dy / dz) as f32,
                    );
                }
            }

            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas
                .draw_line(
                    Point::new(hor_origin as i32, (ver_origin - 10.0) as i32),
                    Point::new(hor_origin as i32, (ver_origin + 10.0) as i32),
                )
                .unwrap();
            canvas
                .draw_line(
                    Point::new((hor_origin - 10.0) as i32, ver_origin as i32),
                    Point::new((hor_origin + 10.0) as i32, ver_origin as i32),
                )
                .unwrap();
        }
    }

    canvas.set_clip_rect(None);
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    unsafe {
        canvas.draw_text(0, 0, &DEBUG_STRING).unwrap();
    }
    canvas.present();
}

pub fn init_players(players: &mut [Player]) {
    for (i, player) in players.iter_mut().enumerate() {
        player.pos[0] = 8.0 * if i & 1 != 0 { -1.0 } else { 1.0 };
        player.pos[1] = 0.0;
        player.pos[2] =
            8.0 * if i & 1 != 0 { -1.0 } else { 1.0 } * if i & 2 != 0 { -1.0 } else { 1.0 };
        player.vel[0] = 0.0;
        player.vel[1] = 0.0;
        player.vel[2] = 0.0;
        player.yaw = 0x20000000
            + if i & 1 != 0 { 0x80000000 } else { 0 }
            + if i & 2 != 0 { 0x40000000 } else { 0 };
        player.pitch = -0x08000000;
        player.radius = 0.5;
        player.height = 1.5;
        player.wasd = 0;
        player.mouse = 0;
        player.keyboard = 0;
        player.color[0] = if (1 << (i / 2)) & 2 != 0 { 0 } else { 0xff };
        player.color[1] = if (1 << (i / 2)) & 1 != 0 { 0 } else { 0xff };
        player.color[2] = if (1 << (i / 2)) & 4 != 0 { 0 } else { 0xff };
        player.color[0] = if i & 1 != 0 {
            player.color[0]
        } else {
            !player.color[0]
        };
        player.color[1] = if i & 1 != 0 {
            player.color[1]
        } else {
            !player.color[1]
        };
        player.color[2] = if i & 1 != 0 {
            player.color[2]
        } else {
            !player.color[2]
        };
    }
}
