use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

use cursive::direction::Orientation;
use cursive::event::{Event, Key};
use cursive::view::Boxable;
use cursive::view::Identifiable;
use cursive::views::*;
use cursive::Cursive;

use hex;

mod nvis;

fn main() {
    let mut transformer_dialogs = Vec::with_capacity(nvis::TRANSFORMERS.len());
    for transformer in nvis::TRANSFORMERS.iter() {
        transformer_dialogs.push(
            Dialog::around(
                TextView::new(nvis::NONE_PLACEHOLDER)
                    .no_wrap()
                    .with_id(transformer.label()),
            )
            .title(transformer.label())
            .with_id(format!("_{}_dialog", transformer.label()).as_str())
            .full_width(),
        )
    }

    let mut left_transformers = LinearLayout::new(Orientation::Vertical);
    let mut right_transformers = LinearLayout::new(Orientation::Vertical);

    for (idx, dialog) in transformer_dialogs.into_iter().enumerate() {
        if idx < nvis::TRANSFORMERS.len() / 2 {
            left_transformers.add_child(dialog)
        } else {
            right_transformers.add_child(dialog)
        }
    }

    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::new(Orientation::Vertical)
                .child(EditView::new().on_edit(update_fields).with_id("_input"))
                .child(
                    LinearLayout::new(Orientation::Horizontal)
                        .child(left_transformers)
                        .child(right_transformers),
                )
                .child(TextView::new("").with_id("_status_bar").full_width()),
        )
        .title("nvis")
        .full_screen(),
    );

    siv.set_user_data(nvis::Context {
        input_mode: nvis::InputMode::Raw,
        focus_idx: 0,
    });

    siv.add_global_callback(Event::Key(Key::Up), focus_down);
    siv.add_global_callback(Event::Key(Key::Down), focus_up);
    siv.add_global_callback(Event::CtrlChar('s'), clip);
    siv.add_global_callback(Event::CtrlChar('t'), toggle_mode);
    siv.add_global_callback(Event::CtrlChar('q'), |s| s.quit());

    tick(&mut siv);
    siv.run();
}

fn tick(s: &mut Cursive) {
    let context = s.user_data::<nvis::Context>().unwrap();

    let input_mode = context.input_mode.clone();
    let focus_idx = context.focus_idx as usize;
    s.call_on_id("_status_bar", |view: &mut TextView| {
        view.set_content(format!("M: {}, I: {}", input_mode, focus_idx));
    });

    for (idx, transformer) in nvis::TRANSFORMERS.iter().enumerate() {
        let title = match idx == focus_idx {
            true => format!("{} (F)", transformer.label()),
            false => String::from(transformer.label()),
        };

        s.call_on_id(
            format!("_{}_dialog", transformer.label()).as_str(),
            |view: &mut Dialog| {
                view.set_title(title.as_str());
            },
        );
    }
}

fn focus_up(s: &mut Cursive) {
    let mut context = s.user_data::<nvis::Context>().unwrap();
    context.focus_idx = context.focus_idx.wrapping_add(1) % (nvis::TRANSFORMERS.len() as u16);
    tick(s);
}

fn focus_down(s: &mut Cursive) {
    let mut context = s.user_data::<nvis::Context>().unwrap();
    context.focus_idx = context.focus_idx.wrapping_sub(1) % (nvis::TRANSFORMERS.len() as u16);
    tick(s);
}

fn clip(s: &mut Cursive) {
    let context = s.user_data::<nvis::Context>().unwrap();
    let mut clipboard: ClipboardContext = clipboard::ClipboardProvider::new().unwrap();

    let focus_id = nvis::TRANSFORMERS[context.focus_idx as usize].label();
    s.call_on_id(focus_id, |view: &mut TextView| {
        clipboard
            .set_contents((*view.get_content()).source().to_string())
            .unwrap();
    });
}

fn toggle_mode(s: &mut Cursive) {
    let mut context = s.user_data::<nvis::Context>().unwrap();

    if context.input_mode == nvis::InputMode::Raw {
        context.input_mode = nvis::InputMode::Smart;
    } else {
        context.input_mode = nvis::InputMode::Raw;
    }
    tick(s);
}

fn drop_leading_zeros(bytes: &[u8]) -> Vec<u8> {
    let mut leading = true;
    let mut bytes_vec = Vec::new();

    for byte in bytes.iter().rev() {
        if *byte != 0 {
            leading = false;
        }

        if !leading {
            bytes_vec.push(*byte);
        }
    }

    bytes_vec
}

fn build_input(input: &str, mode: &nvis::InputMode) -> Vec<u8> {
    if input.is_empty() {
        return String::from("").into_bytes();
    }

    if mode == &nvis::InputMode::Raw {
        String::from(input).into_bytes()
    } else {
        if input.starts_with("0x") {
            match hex::decode(&input[2..]) {
                Ok(v) => return v,
                Err(_e) => return vec![],
            }
        } else if input.starts_with("0o") {
            match isize::from_str_radix(&input[2..], 8) {
                Ok(n) => return drop_leading_zeros(&n.to_ne_bytes()),
                Err(_e) => return vec![],
            }
        } else if input.starts_with("0b") {
            match isize::from_str_radix(&input[2..], 2) {
                Ok(n) => return drop_leading_zeros(&n.to_ne_bytes()),
                Err(_e) => return vec![],
            }
        } else {
            String::from(input).into_bytes()
        }
    }
}

fn update_fields(s: &mut Cursive, input: &str, _size: usize) {
    let context = s.user_data::<nvis::Context>().unwrap();
    let built_input = build_input(input, &context.input_mode);
    for transformer in nvis::TRANSFORMERS.iter() {
        s.call_on_id(transformer.label(), |view: &mut TextView| {
            view.set_content(transformer.transform(built_input.as_slice()));
        });
    }
}
