extern crate termbox;

use termbox::{
    Termbox,
    Attribute,
    Cell,
    BOLD,
};

use crate::Section;


pub fn draw_fn(tb: &mut Termbox, fg: &Attribute, bg: &Attribute) {
    tb.put_str(1, 0, "F1 shutdown", *fg | BOLD, *bg);
    tb.put_str(13, 0, "F2 reboot", *fg | BOLD, *bg);
}

pub fn draw_box(tb: &mut Termbox, width: &i32, height: &i32, x: &i32, y: &i32, x2: &i32, y2: &i32, fg: &Attribute, bg: &Attribute, hl: &Cell, vl: &Cell, lul: &Cell, ldl: &Cell, rul: &Cell, rdl: &Cell) {
    // draw horizontal lines
    for i in 1..*width {
        tb.put_cell(*x + i, *y, *hl);
        tb.put_cell(*x + i, *y2, *hl);
    }
    // draw vertical lines
    for i in 1..*height {
        tb.put_cell(*x, *y + i, *vl);
        tb.put_cell(*x2, *y + i, *vl);
    }

    // draw corners
    tb.put_cell(*x, *y, *lul);
    tb.put_cell(*x, *y2, *ldl);
    tb.put_cell(*x2, *y, *rul);
    tb.put_cell(*x2, *y2, *rdl);

    // draw launcher field
    tb.put_cell(*x + 12, *y2 - 6, Cell {ch: '<' as u32, fg: *fg, bg: *bg} );
    tb.put_cell(*x2 - 3, *y2 - 6, Cell {ch: '>' as u32, fg: *fg, bg: *bg} );


    // draw username field
    tb.put_str(*x + 3 , *y2 - 4, "login", *fg, *bg);
    tb.put_str(*x + 3, *y2 - 2, "password" , *fg, *bg);
}

pub fn update(tb: &mut Termbox, width: &i32, height: &i32, x: &i32, y: &i32, x2: &i32, y2: &i32, fg: &Attribute, bg: &Attribute, username: &String, password: &String, index: &i32, section: &Section, px: &mut i32, py: &mut i32, pch: &mut char) {

    match *section {
        Section::LAUNCHER => {
            if *pch == '\u{2588}' {
                tb.change_cell(*px, *py, *pch, *bg, *bg);
            } else {
                tb.change_cell(*px, *py, *pch, *fg, *bg);
            }

            // tb.change_cell(*px, *py, *pch, *fg, *bg);
            tb.put_str(*x + 14, *y2 - 6, "xinitrc", *fg, *bg);
            *px = *x + 12;
            *py = *y2 - 6;
            *pch = '<';
            tb.change_cell(*px, *py, *pch, *bg, *fg)
        },
        Section::LOGIN => draw_login(tb, width, height, x, y, x2, y2, fg, bg, username, index, px, py, pch),
        Section::PASSWORD => {},
    }
}

// fn rm_highlights(tb: &mut Termbox, width: &i32, height: &i32, x: &i32, y: &i32, x2: &i32, y2: &i32, fg: &Attribute, bg: &Attribute, username: &String, password: &String, index: Option<i32>) {

fn draw_login(tb: &mut Termbox, width: &i32, height: &i32, x: &i32, y: &i32, x2: &i32, y2: &i32, fg: &Attribute, bg: &Attribute, username: &String, index: &i32, px: &mut i32, py: &mut i32, pch: &mut char) {
    // reset current login
    for i in (*x + 12)..(*x2) {
        tb.change_cell(i, *y2-4, '\u{2588}', *bg, *bg);
    }

    tb.put_str(*x + 12, *y2 - 4, username, *fg, *bg);
    let n = username.len() as i32;
    if *py != (*y2 - 4) {
        tb.change_cell(*px, *py, *pch, *fg, *bg);
    }
    // if *pch == '\u{2588}' {
    //     tb.change_cell(*px, *py, *pch, *bg, *bg);
    // } else {
    //     tb.change_cell(*px, *py, *pch, *fg, *bg);
    // }
    if *index < n  {
        *px = *x + 12 + *index;
        *py = *y2-4;
        *pch = username.chars().nth(*index as usize).unwrap(); 
        // tb.put_cell(*px, *py, Cell { ch: *pch, fg: *bg, bg: *fg } );
        tb.change_cell(*px, *py, *pch, *bg, *fg);

        // to get rid of extra highlight
        // not ideal
        // tb.put_cell(*x + 12 + n, *y2-4, Cell { ch: 0x2588, fg: *bg, bg: *bg } );
    } else {
        *px = *x + 12 + n;
        *py = *y2 - 4;
        *pch = '\u{2588}';
        // tb.put_cell(*px, *py, Cell { ch: *pch, fg: *fg, bg: *bg } );
        tb.change_cell(*px, *py, *pch, *fg, *bg);

    }
}
// pub fn draw_input(tb: &mut,
