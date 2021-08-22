/*
  Copyright (c) 2021 Ergpopler
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Button};
use gtk::prelude::BoxExt;
use gdk;
use glib;

use webkit2gtk::{WebContext, WebView};
use webkit2gtk::traits::WebViewExt;

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
	webview.load_uri("https://duckduckgo.com");
	let back_button = Button::builder()
		.label("Back")
		.margin_top(0)
		.margin_bottom(0)
		.margin_start(0)
		.margin_end(0)
		.build();

	let next_button = Button::builder()
		.label("Next")
		.margin_top(0)
		.margin_bottom(0)
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
	vbox.pack_start(&back_button, false, false, 5);
	vbox.pack_start(&next_button, false, false, 5);
	vbox.pack_start(&webview, true, true, 0);
	window.show_all();

	window.present();

}
