use ratatui::{
	prelude::{Buffer, Rect},
	widgets::Widget,
};

use crate::model::account::{Account, AccountRepository};

impl Widget for Account {
	fn render(self, area: Rect, buf: &mut Buffer)
	where
		Self: Sized,
	{
	}
}

impl Widget for AccountRepository {
	fn render(self, area: Rect, buf: &mut Buffer)
	where
		Self: Sized,
	{
	}
}
