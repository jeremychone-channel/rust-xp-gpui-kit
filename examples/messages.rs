use gpui_kit::base::StyledExt;
use gpui_kit::component::button::Button;
use gpui_kit::*;

struct MessageStore {
	count: usize,
}

struct MessagePanel {
	store: Entity<MessageStore>,
	_subscription: Subscription,
}

impl MessagePanel {
	fn new(cx: &mut Context<Self>) -> Self {
		let store = cx.new(|_| MessageStore { count: 0 });
		let _subscription = cx.observe(&store, |_, _, cx| cx.notify());
		Self { store, _subscription }
	}
}

impl Render for MessagePanel {
	fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
		let count = self.store.read(cx).count;

		div()
			.v_flex()
			.gap_2()
			.size_full()
			.items_center()
			.justify_center()
			.child(format!("Messages: {count}"))
			.child(
				Button::new("add-message")
					.label("Add message")
					.on_click(cx.listener(|this, _, _, cx| {
						this.store.update(cx, |store, cx| {
							store.count += 1;
							cx.notify();
						});
					})),
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
