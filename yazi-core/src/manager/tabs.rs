use yazi_config::BOOT;
use yazi_shared::fs::Url;

use crate::{manager::Manager, tab::Tab};

pub struct Tabs {
	pub idx:          usize,
	pub(super) items: Vec<Tab>,
}

impl Tabs {
	pub fn make() -> Self {
		let mut tabs = Self { idx: 0, items: vec![Tab::from(Url::from(&BOOT.cwd))] };
		if let Some(file) = &BOOT.file {
			tabs.items[0].reveal(Url::from(BOOT.cwd.join(file)));
		}

		Manager::_refresh();
		tabs
	}

	#[inline]
	pub(super) fn absolute(&self, rel: isize) -> usize {
		if rel > 0 {
			(self.idx + rel as usize).min(self.items.len() - 1)
		} else {
			self.idx.saturating_sub(rel.unsigned_abs())
		}
	}

	#[inline]
	pub(super) fn set_idx(&mut self, idx: usize) {
		if self.idx == idx {
			return;
		}

		// Reset the preview of the previous active tab
		if let Some(active) = self.items.get_mut(self.idx) {
			active.preview.reset_image();
		}

		self.idx = idx;
		Manager::_refresh();
		Manager::_peek(true);
	}
}

impl Tabs {
	#[inline]
	pub fn len(&self) -> usize { self.items.len() }

	#[inline]
	pub fn iter(&self) -> impl Iterator<Item = &Tab> { self.items.iter() }

	#[inline]
	pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tab> { self.items.iter_mut() }

	#[inline]
	pub fn active(&self) -> &Tab { &self.items[self.idx] }

	#[inline]
	pub(super) fn active_mut(&mut self) -> &mut Tab { &mut self.items[self.idx] }
}
