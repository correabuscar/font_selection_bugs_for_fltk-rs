// FLTK offers 16 fonts by default. However, it's possible to load all system fonts to be able to choose from them:
// The following are the default FLTK fonts:
// - Helvetica,
// - HelveticaBold,
// - HelveticaItalic,
// - HelveticaBoldItalic,
// - Courier,
// - CourierBold,
// - CourierItalic,
// - CourierBoldItalic,
// - Times,
// - TimesBold,
// - TimesItalic,
// - TimesBoldItalic,
// - Symbol,
// - Screen,
// - ScreenBold,
// - Zapfdingbats,
//
// The system fonts depend on the system, and are not loaded by default.
// These can be loaded using the App::load_system_fonts() method.
// The fonts can then be acquired using the app::fonts() function
// or be queried using the app::font_count(), app::font_name() and app::font_index() functions.
// And then can be used using the Font::by_index() or Font::by_name() methods.

use fltk::enums::Align;
use fltk::{prelude::*, *};
use fltk::app::App;



fn setup(title:&str,load_sys_fonts:bool, set_app_default_font:bool, use_index:usize) -> App {
    let app = app::App::default();

    if load_sys_fonts {
        let app = app.load_system_fonts();
    }

    app::set_background_color(0, 0, 0);

    let fonts = app::fonts();

    let mut wind = window::Window::default().with_size(400, 300);
    wind.make_resizable(true);
    let mut frame = frame::Frame::default().size_of(&wind);
    frame.set_label_size(20); //font size, lookslike!
    frame.set_label_color(enums::Color::White);

    if set_app_default_font {
        app::set_font(enums::Font::CourierBold);
        //app::set_font(enums::Font::HelveticaBold);
        //app::set_font(enums::Font::by_name("BDejaVu Sans Mono"));
        //app::set_font(enums::Font::Helvetica);
        //app::set_font(enums::Font::by_name("BArial"));
    }

    wind.end();
    wind.show();

    let index = use_index; //a bug happens when index is 0, but yet another bug is still visible.
    //let index=2;//the first bug goes away when index >0, but the second bug is still visible now
    println!("The system has {} fonts! First font is(these should all be equal,right?): [{:?}] [{}] [{}] [{}]",
             fonts.len(),
             enums::Font::by_index(index),
             fonts[index],
             app::fonts()[index], // just making sure this is still fonts[index], and it is !
             app::get_font_names()[index],

    );

    let font_by_index = enums::Font::by_index(index);
    frame.set_align(Align::Wrap);
    frame.set_label_font(font_by_index);


    let first_font = &fonts[index];
    frame.set_label(&format!(
        "{}\nThis font at index[{}] is not [{:?}], even though it should be \
         [{}], but it's rather the app default font [{}] \
         unless app::set_font(...) was never called!",
        title,
        index,
        font_by_index,
        first_font,
        app::get_font_names()[index],
    ));
    //frame.set_label_font(fi);//doubly sure?

    return app;
}

//FIXME: find out why can't run both tests, even sequentially! (the second test hangs!)  cargo test -- --test-threads=1
#[test]
fn one_bug() {
    app::sleep(1.);
    setup("test - bug1", true, true, 0).wait();
    app::sleep(1.);
}


#[test]
fn another_bug(){
    app::sleep(2.);
    setup("test - bug2",true,true,2).wait();
    app::sleep(2.);
}

fn main() {
    setup("main - bug1",true,true,0).run().unwrap();
    setup("main - bug2",true,true,2).run().unwrap();
}
