/*
  Copyright (c) 2021 Ergpopler
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/



use gtk::prelude::*;
use gtk::Align;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Button};
use gtk::prelude::{WidgetExt, ButtonExt, BoxExt};

use webkit2gtk::WebView;
use webkit2gtk::traits::WebViewExt;

mod tabs;

fn main() {
	gtk::init().unwrap();
	let app = Application::builder()
		.application_id("io.github.murasaki")
		.build();

	app.connect_activate(build_ui);

	app.run();
}

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("Murasaki")
		.build();
	let vbox = Box::builder()
		.margin_top(0)
		.margin_bottom(0)
		.margin_start(0)
		.margin_end(0)
		.expand(true)
		.orientation(gtk::Orientation::Vertical)
		.build();
	let hbox = Box::builder()
		.margin_top(0)
		.margin_bottom(0)
		.margin_start(0)
		.margin_end(0)
		.expand(false)
		.orientation(gtk::Orientation::Horizontal)
		.halign(Align::Start)
		.build();

	let back_button = Button::builder()
		.label("<")
		.margin_top(0)
		.margin_bottom(0)
		.margin_start(0)
		.margin_end(0)
		.expand(false)
		.build();

	let next_button = Button::builder()
		.label(">")
		.margin_top(0)
		.margin_bottom(0)
		.margin_start(0)
		.margin_end(0)
		.expand(false)
		.build();
	hbox.pack_start(&back_button, false, false, 0);
	hbox.pack_start(&next_button, false, false, 0);

    let (view, bar) = tabs::make_tab_bar();

    vbox.add(&bar);
    vbox.add(&hbox);
    vbox.add(&view);

	window.add(&vbox);

    view.connect_selected_page_notify(move |_x|{
        let i = _x.selected_page();
        let i = match i {
            Some(s) => s,
            None => std::process::exit(0),
        };
        let webview = i.child();
        let webview = match webview {
            Some(s) => s.downcast::<WebView>().unwrap(),
            None => panic!("Child wasn't a webview, weird."),
        };

    	let webview_next = webview.clone();
    	next_button.connect_clicked(move |_x| {
    		webview_next.go_forward();
    	});
    	let webview_back = webview.clone();
    	back_button.connect_clicked(move |_y| {
    		webview_back.go_back();
    	});

    });

    tabs::append_new_tab(&view.clone());

	window.show_all();

	window.present();
	let screen = window.screen();
	let screen = match screen {
		Some(s) => s,
		None => panic!("No Screen Found"),
	};

    #[allow(deprecated)]
	let screen_size = (screen.width(), screen.height());
	window.connect_configure_event(move |x, _y| {
		if x.size() == screen_size {
			hbox.set_visible(false);
			bar.set_visible(false);
		} else {
			hbox.set_visible(true);
			bar.set_visible(true);
		}
		false
	});

	// The following is just for testing purposes, really.
	/*
	window.connect_key_press_event(move |_x, y| {
		if let state = y.state() {
			println!("Control key pressed 😳: {}", state);
		}
		let keyvalue = y.keyval().to_unicode();
		match keyvalue {
			Some(v) => {
				println!("Key Pressed 😳: {}", v)
			}
			None => {}
		};

		Inhibit(false)
	});
	*/
}
