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
    KEY_ARROW_UP,
    KEY_ARROW_DOWN,
    KEY_ARROW_LEFT,
    KEY_ARROW_RIGHT,
};

use rustLy_conf::configs;
use crate::{
    draw,
    Section,
};


fn up_section(s: &mut Section) {
    *s = match *s {
        Section::LAUNCHER => Section::PASSWORD,
        Section::LOGIN => Section::LAUNCHER,
        Section::PASSWORD => Section::LOGIN,
    }
}

fn down_section(s: &mut Section) {
    *s = match *s {
        Section::LAUNCHER => Section::LOGIN,
        Section::LOGIN => Section::PASSWORD,
        Section::PASSWORD => Section::LAUNCHER,
    }
}

pub fn launch() {
    let colors: [termbox::Attribute; 8] = [BLACK, WHITE, BLUE, CYAN, GREEN, MAGENTA, RED, YELLOW];

    // let LETTERS: HashSet<char> = vec![
    //     'a', 'A', 'b', 'B', 'c', 'C', 'd', 'D', 'e', 'E', 'f', 'F', 'g', 'G', 'h', 'H', 'i', 'I', 'j', 'J', 
    //     'k', 'K', 'l', 'L', 'm', 'M', 'n', 'N', 'O'
    // ].into_iter().collect();

    let mut username = String::new();
    username.push_str("slarch");
    let mut password = String::new();
    let mut section: Section = Section::LOGIN;
    let mut needs_update: bool = true;
    let mut index = 0;
    let mut password_len = 0;

    // first load configs
    let conf: configs::Config = configs::get_configs();

    let fg = colors[conf.fg()];
    let bg = colors[conf.bg()];

    let pchar = conf.asterisk();
    let box_height = 8 + (2 * conf.margin_box_v());
    let box_width = (2 * conf.margin_box_h()) + conf.input_len() + 1 + 8;


    let mut tb = Termbox::open().unwrap();

    let width = tb.width();
    let height = tb.height();

    tb.set_clear_attributes(fg, bg);
    tb.clear();

    draw::draw_fn(&mut tb, &fg, &bg);
    // let box_width: c_int = (width as f32 * 0.8) as c_int;
    // let box_height: c_int = (height as f32 * 0.33) as c_int;
    let box_x = (width - box_width) / 2;
    let box_x2 = (width + box_width) / 2;

    let box_y = (height - box_height) / 2;
    let box_y2 = (height + box_height) / 2;

    // keep track of previous to unhighlight
    let mut prev_x = box_x + 12;
    let mut prev_y = box_y2 - 4;
    let mut prev_char = '\u{2588}';

    let horizontal_line: termbox::Cell = termbox::Cell { ch: 0x2500, fg: fg, bg: bg };
    let vertical_line: termbox::Cell = termbox::Cell { ch: 0x2502, fg: fg, bg: bg };
    let left_up_line: termbox::Cell = termbox::Cell { ch: 0x250c, fg: fg, bg: bg };
    let left_down_line: termbox::Cell = termbox::Cell { ch: 0x2514, fg: fg, bg: bg };
    let right_up_line: termbox::Cell = termbox::Cell { ch: 0x2510, fg: fg, bg: bg };
    let right_down_line: termbox::Cell = termbox::Cell { ch: 0x2518, fg: fg, bg: bg };

    let password_cell: termbox::Cell = termbox::Cell { ch: pchar as u32, fg: fg, bg: bg };

    draw::draw_box(&mut tb, &box_width, &box_height, &box_x, &box_y, &box_x2, &box_y2, &fg, &bg, &horizontal_line, &vertical_line, &left_up_line, &left_down_line, &right_up_line, &right_down_line);
    draw::update(&mut tb, &box_width, &box_height, &box_x, &box_y, &box_x2, &box_y2, &fg, &bg, &username, &password_cell, &password_len, &index, &section, &mut prev_x, &mut prev_y, &mut prev_char);

    tb.present();
    loop {
        match tb.poll_event() {
            Event::Key(event) => {
                if event.key == KEY_ESC {
                    break;
                }
                else if event.key == KEY_F1 {
                    break;
                }
                else if event.key == KEY_F2 {
                    break;
                }
                else if event.key == KEY_ARROW_UP {
                    up_section(&mut section);
                    index = 0;
                    needs_update = true;
                }
                else if event.key == KEY_ARROW_DOWN {
                    down_section(&mut section);
                    index = 0;
                    needs_update = true;
                }
                else if event.key == termbox::KEY_BACKSPACE || event.key == termbox::KEY_BACKSPACE2 {
                    match section {
                        Section::LOGIN => {
                            if index > 0 {
                                index = index - 1;
                                username.remove(index as usize);
                                needs_update = true;
                            }
                        },
                        Section::PASSWORD => {
                            if index > 0 {
                                index = index - 1;
                                password.remove(index as usize);
                                password_len = password_len - 1;
                                needs_update = true;
                            }
                        },
                        _ => {},
                    }
                }
 
                else if event.key == KEY_ARROW_LEFT {
                    match section {
                        Section::LAUNCHER => {},
                        _ => {
                            if index > 0 {
                                index = index - 1;
                            }
                        }
                    }
                    needs_update = true;
                }
                else if event.key == KEY_ARROW_RIGHT {
                    match section {
                        Section::LAUNCHER => {},
                        Section::LOGIN => { if index < (username.len() as i32) { index = index + 1; } },
                        Section::PASSWORD => { if index < (password.len() as i32) { index = index + 1; } },
                    }
                    needs_update = true;
                } else {
                    match event.ch {
                        None => {},
                        Some(c) => {
                            if c > (32 as char) && c < (127 as char) {
                                match section {
                                    Section::LAUNCHER => {},
                                    Section::LOGIN => {
                                        username.insert(index as usize, c);
                                        index = index + 1;
                                        needs_update = true;
                                    },
                                    Section::PASSWORD => {
                                        password.insert(index as usize, c);
                                        password_len = password_len + 1;
                                        index = index + 1;
                                        needs_update = true;
                                    },
                                }
                            }
                        },
                    }
                }
                
                if needs_update {
                    draw::update(&mut tb, &box_width, &box_height, &box_x, &box_y, &box_x2, &box_y2, &fg, &bg, &username, &password_cell, &password_len, &index, &section, &mut prev_x, &mut prev_y, &mut prev_char);
                    tb.present();
                    needs_update = false;
                }

            },
            _ => {},
       }
    }
}
