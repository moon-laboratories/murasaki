/*
  Copyright (c) 2021 Ergpopler
  This Source Code Form is subject to the terms of the Mozilla Public
  License, v. 2.0. If a copy of the MPL was not distributed with this
  file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use gtk::prelude::*;
use gtk::Align;
use gtk::{Application, ApplicationWindow};
use gtk::{Box, Button, Entry};
use gtk::prelude::{WidgetExt, ButtonExt, BoxExt};

use webkit2gtk::WebView;
use webkit2gtk::traits::WebViewExt;

mod tabs;

fn main() {
	gtk::init().expect("Failed to initialize GTK");
	libhandy::init();
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
		.default_height(480)
		.default_width(700)
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
	let screen = window.screen();
	let screen = match screen {
		Some(s) => s,
		None => panic!("No Screen Found"),
	};

	#[allow(deprecated)]
	let screen_size = (screen.width(), screen.height());

	let url_bar = Entry::builder()
		.editable(true)
		.expand(true)
		.max_width_chars(screen_size.0)
		.build();
	hbox.pack_start(&back_button, false, false, 0);
	hbox.pack_start(&next_button, false, false, 0);
	hbox.add(&url_bar);

	let (view, bar) = tabs::make_tab_bar();

	vbox.add(&bar);
	vbox.add(&hbox);
	vbox.add(&view);

	window.add(&vbox);

	view.connect_selected_page_notify(move |_x| {
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

		tabs::autoset_title(&webview, i.clone());

		let enter_url_bar = url_bar.clone();

		let enter_webview = webview.clone();
		enter_url_bar.connect_activate(move |x| {
			let strcont = x.text();
			let mut goto_url: String = "".to_owned();
			if strcont == "" {
				goto_url.push_str("about:blank")
			} else if strcont.starts_with("https://") | strcont.starts_with("http://") {
				goto_url = strcont.to_string();
			} else {
				let query = str::replace(&strcont, " ", "+");
				goto_url.push_str(&format!("https://duckduckgo.com/?q={}", query));
			}
			enter_webview.load_uri(&goto_url);
		});

		let url = WebViewExt::uri(&webview);
		let url = match url {
			Some(s) => s,
			None => panic!("No URL"),
		};
		enter_url_bar.set_text(url.as_str());

		let webview_next = webview.clone();
		next_button.connect_clicked(move |_x| {
			webview_next.go_forward();
		});
		let webview_back = webview.clone();
		back_button.connect_clicked(move |_y| {
			webview_back.go_back();
		});
		let load_url_bar = url_bar.clone();
		load_url_bar.set_progress_fraction(0.0);
		webview.connect_load_changed(move |slf, _y| {
			let url = WebViewExt::uri(slf);
			let url = match url {
				Some(s) => s,
				None => panic!("No URL"),
			};
			load_url_bar.set_text(url.as_str());

			tabs::autoset_title(&slf, i.clone());

			let load_percentage = WebViewExt::estimated_load_progress(slf);
			if load_percentage != 1.0 {
				load_url_bar.set_progress_fraction(load_percentage);
			} else {
				load_url_bar.set_progress_fraction(0.0);
			}
		});
	});

	tabs::append_new_tab(&view.clone());

	window.show_all();

	window.present();
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
}
