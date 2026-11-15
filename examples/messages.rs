use gpui_kit::base::StyledExt;
use gpui_kit::component::button::Button;
use gpui_kit::*;

struct MessageStore {
	count: usize,
}

struct MessagePanel {
	store: Entity<MessageStore>,
	// required to not drop subscription
	_sub: Subscription,

	// experiment to see if we can UI component state
	component_ui_stuff: String,
}

impl MessagePanel {
	fn new(cx: &mut Context<Self>) -> Self {
		let store = cx.new(|_| MessageStore { count: 0 });
		let _sub = cx.observe(&store, |_, _, cx| cx.notify());
		Self {
			store,
			_sub,
			component_ui_stuff: "hello someting".to_string(),
		}
	}
}

impl Render for MessagePanel {
	fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
		println!("->> MessagePanel RENDER");

		let count = self.store.read(cx).count;

		div()
			.v_flex()
			.gap_2()
			.size_full()
			.items_center()
			.justify_center()
			.child(format!("Messages: {count}"))
			.child(
				Button::new("add-message").label("Add message").on_click(cx.listener(
					|this, _evt, _win, cx| {
						println!("->> BUTTON CLICK");
						this.component_ui_stuff = "something else".to_string();

						this.store.update(cx, |store, cx| {
							println!("->> STORE UPDATE  (prev: {})", store.count);
							store.count += 1;
							println!("->> STORE UPDATE NOTIFY PRE");
							cx.notify();
							println!("->> STORE UPDATE NOTIFY POST");
						});
					},
				)),
			)
	}
}

fn main() {
	gpui_kit::application().run(|cx| {
		gpui_kit::init(cx);
		gpui_kit::open_window(WindowOptions::default(), cx, |_, cx| cx.new(MessagePanel::new))
			.expect("failed to open window");
	});
}
