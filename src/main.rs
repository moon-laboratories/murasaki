/*
  Copyright (c) 2021 Ergpopler
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Button, Inhibit};
use gtk::prelude::{WidgetExt, ButtonExt, BoxExt};

use webkit2gtk::{WebContext, WebView};
use webkit2gtk::traits::{WebViewExt, WindowPropertiesExt};

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
	let context = WebContext::default().unwrap();
	let webview = WebView::with_context(&context);
	let vbox = Box::builder()
		.margin_top(0)
		.margin_bottom(0)
		.margin_start(4)
		.margin_end(0)
		.build();

	window.add(&vbox);
	webview.load_uri("https://duck.com");
	let back_button = Button::builder()
		.label("Back")
		.margin_top(0)
		.margin_bottom(1000)
		.margin_start(0)
		.margin_end(0)
		.build();

	let next_button = Button::builder()
		.label("Next")
		.margin_top(0)
		.margin_bottom(1000)
		.margin_start(0)
		.margin_end(0)
		.build();

	let webview_next = webview.clone();
	next_button.connect_clicked(move |_x| {
		webview_next.go_forward();
	});
	let webview_back = webview.clone();
	back_button.connect_clicked(move |_y| {
		webview_back.go_back();
	});
	vbox.pack_start(&back_button, false, true, 3);
	vbox.pack_start(&next_button, false, true, 3);
	vbox.pack_start(&webview, true, true, 0);
	window.show_all();

	window.present();
	let screen = window.screen();
	let screen = match screen {
		Some(s) => s,
		None => panic!("No Screen Found"),
	};
	let screen_size = (screen.width(), screen.height());
	window.connect_configure_event(move |x, _y| {
		if x.size() == screen_size {
			back_button.set_visible(false);
			next_button.set_visible(false);
		} else {
			back_button.set_visible(true);
			next_button.set_visible(true);
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
