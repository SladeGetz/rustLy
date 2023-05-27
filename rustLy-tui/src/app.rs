extern crate termbox;

use std::ffi::c_int;

use termbox::{
    Termbox,
    Event,
    BLACK,
    WHITE,
    BLUE,
    CYAN,
    GREEN,
    MAGENTA,
    RED,
    YELLOW,
    BOLD,
    KEY_ESC,
    KEY_F1,
    KEY_F2,
};

use rustLy_conf::configs;

pub fn launch() {
    let colors: [termbox::Attribute; 8] = [BLACK, WHITE, BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW];

    // first load configs
    let conf: configs::Config = configs::getConfigs();

    let mut tb = Termbox::open().unwrap();

    tb.set_clear_attributes(colors[conf.fg()], colors[conf.bg()]);
    tb.clear();

    tb.put_str(0, 0, "F1 shutdown", colors[conf.fg()] | BOLD, colors[conf.bg()]);
    tb.put_str(12, 0, "F2 reboot", colors[conf.fg()] | BOLD, colors[conf.bg()]);
    let width = tb.width();
    let height = tb.height();

    // let box_width: c_int = (width as f32 * 0.8) as c_int;
    // let box_height: c_int = (height as f32 * 0.33) as c_int;
    let box_width: c_int = 45;
    let box_height: c_int = 9;

    let box_x: c_int = (width - box_width) / 2;
    let box_x2: c_int = (width + box_width) / 2;

    let box_y: c_int = (height - box_height) / 2;
    let box_y2: c_int = (height + box_height) / 2;

    let horizontal_line: termbox::Cell = termbox::Cell { ch: 0x2500, fg: colors[conf.fg()], bg: colors[conf.bg()] };
   let vertical_line: termbox::Cell = termbox::Cell { ch: 0x2502, fg: colors[conf.fg()], bg: colors[conf.bg()] };
    let left_up_line: termbox::Cell = termbox::Cell { ch: 0x250c, fg: colors[conf.fg()], bg: colors[conf.bg()] };
    let left_down_line: termbox::Cell = termbox::Cell { ch: 0x2514, fg: colors[conf.fg()], bg: colors[conf.bg()] };
    let right_up_line: termbox::Cell = termbox::Cell { ch: 0x2510, fg: colors[conf.fg()], bg: colors[conf.bg()] };
    let right_down_line: termbox::Cell = termbox::Cell { ch: 0x2518, fg: colors[conf.fg()], bg: colors[conf.bg()] };

    // draw horizontal lines
    for i in 1..box_width {
        tb.put_cell(box_x + i, box_y, horizontal_line);
        tb.put_cell(box_x + i, box_y2, horizontal_line);
    }
    // draw vertical lines
    for i in 1..box_height {
        tb.put_cell(box_x, box_y + i, vertical_line);
        tb.put_cell(box_x2, box_y + i, vertical_line);
    }

    // draw corners
    tb.put_cell(box_x, box_y, left_up_line);
    tb.put_cell(box_x, box_y2, left_down_line);
    tb.put_cell(box_x2, box_y, right_up_line);
    tb.put_cell(box_x2, box_y2, right_down_line);

    tb.present();
    loop {
        match tb.poll_event() {
            Event::Key(event) => {
                if event.key == KEY_ESC {
                    break;
                }
            },
            _ => {},
        }
    }
}
