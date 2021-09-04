use gtk::Button;
use gtk::prelude::{WidgetExt, ButtonExt};

use webkit2gtk::WebView;
use webkit2gtk::WebContext;
use webkit2gtk::traits::WebViewExt;

use libhandy::{TabBar, TabView};

fn make_new_tab(context: WebContext) -> WebView {
	let webview = WebView::with_context(&context);
	webview.load_uri("https://duckduckgo.com");
	webview.show();
	webview
}

pub fn append_new_tab(vc: &TabView) {
	let new_context = WebContext::default().unwrap();
	let temp_ctx = make_new_tab(new_context);
	vc.append(&temp_ctx);
}

pub fn make_tab_bar() -> (TabView, TabBar) {
	let bar = TabBar::builder()
		.autohide(false)
		.expand(false)
		.expand_tabs(false)
		.build();
	let plus_button = Button::builder().label("+").build();
	let view = TabView::builder().border_width(0).expand(true).build();
	bar.set_end_action_widget(Some(&plus_button));

	bar.set_view(Some(&view));
	view.connect_page_attached(|_x, y, z| {
		y.set_title(Some(&format!("Page {}", z + 1)));
	});
	let vc = view.clone();
	plus_button.connect_clicked(move |_x| {
		append_new_tab(&vc);
	});

	return (view, bar);
}
