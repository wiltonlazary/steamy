extern crate steamy_controller as controller;
use controller::button;
use controller::sound::Note;

use std::time::Duration;
use std::ops::Deref;

trait Update {
	fn update(&mut self, buttons: controller::Button, pad: controller::Pad);
	fn has_update(&self) -> bool;
}

trait Octave {
	fn get(&self) -> u8;
	fn set(&mut self, direction: bool);
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Button {
	A,
	B,
	C,
	D,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Mode {
	Octave(bool),
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct State {
	pub button:  Option<Button>,
	pub trigger: bool,
	pub grip:    bool,
	pub bumper:  bool,
	pub mode:    Option<Mode>,
}

impl Default for State {
	fn default() -> Self {
		State {
			button:  None,
			trigger: false,
			grip:    false,
			bumper:  false,
			mode:    None,
		}
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Right {
	previous: State,
	current:  State,

	octave: u8,
}

impl Default for Right {
	fn default() -> Self {
		Right {
			previous: Default::default(),
			current:  Default::default(),

			octave: 6,
		}
	}
}

impl Update for Right {
	fn update(&mut self, buttons: controller::Button, pad: controller::Pad) {
		self.previous = self.current;

		if !pad.right.is_empty() {
			let x = pad.right.x;
			let y = pad.right.y;

			if y < -15_000 && x > -15_000 && x < 15_000 {
				self.current.button = Some(Button::A);
			}
			else if x < -15_000 && y > -15_000 && y < 15_000 {
				self.current.button = Some(Button::B);
			}
			else if y > 15_000 && x > -15_000 && x < 15_000 {
				self.current.button = Some(Button::C);
			}
			else if x > 15_000 && y > -15_000 && y < 15_000 {
				self.current.button = Some(Button::D);
			}
			else {
				self.current.button = None;
			}
		}
		else {
			if buttons.contains(button::A) {
				self.current.button = Some(Button::A);
			}
			else if buttons.contains(button::X) {
				self.current.button = Some(Button::B);
			}
			else if buttons.contains(button::Y) {
				self.current.button = Some(Button::C);
			}
			else if buttons.contains(button::B) {
				self.current.button = Some(Button::D);
			}
			else {
				self.current.button = None;
			}
		}

		self.current.grip    = buttons.contains(button::RIGHT_GRIP);
		self.current.bumper  = buttons.contains(button::RIGHT_BUMPER);
		self.current.trigger = buttons.contains(button::RIGHT_TRIGGER);

		if buttons.contains(button::FORWARD) {
			if self.current.trigger {
				self.current.mode = Some(Mode::Octave(false));
			}
			else {
				self.current.mode = Some(Mode::Octave(true));
			}
		}
		else {
			self.current.mode = None;
		}
	}

	fn has_update(&self) -> bool {
		self.previous != self.current
	}
}

impl Octave for Right {
	fn get(&self) -> u8 {
		self.octave
	}

	fn set(&mut self, direction: bool) {
		if direction {
			self.octave += 1;

			if self.octave > 9 {
				self.octave = 3;
			}
		}
		else {
			self.octave -= 1;

			if self.octave < 3 {
				self.octave = 9;
			}
		}
	}
}

impl Deref for Right {
	type Target = State;

	fn deref(&self) -> &Self::Target {
		&self.current
	}
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Left {
	previous: State,
	current:  State,

	octave: u8,
}

impl Default for Left {
	fn default() -> Self {
		Left {
			previous: Default::default(),
			current:  Default::default(),

			octave: 6,
		}
	}
}

impl Update for Left {
	fn update(&mut self, buttons: controller::Button, pad: controller::Pad) {
		self.previous = self.current;

		if !pad.left.is_empty() {
			let x = pad.left.x;
			let y = pad.left.y;

			if y < -15_000 && x > -15_000 && x < 15_000 {
				self.current.button = Some(Button::A);
			}
			else if x < -15_000 && y > -15_000 && y < 15_000 {
				self.current.button = Some(Button::B);
			}
			else if y > 15_000 && x > -15_000 && x < 15_000 {
				self.current.button = Some(Button::C);
			}
			else if x > 15_000 && y > -15_000 && y < 15_000 {
				self.current.button = Some(Button::D);
			}
			else {
				self.current.button = None;
			}
		}
		else {
			self.current.button = None;
		}

		self.current.grip    = buttons.contains(button::LEFT_GRIP);
		self.current.bumper  = buttons.contains(button::LEFT_BUMPER);
		self.current.trigger = buttons.contains(button::LEFT_TRIGGER);

		if buttons.contains(button::BACK) {
			if self.current.trigger {
				self.current.mode = Some(Mode::Octave(false));
			}
			else {
				self.current.mode = Some(Mode::Octave(true));
			}
		}
		else {
			self.current.mode = None;
		}
	}

	fn has_update(&self) -> bool {
		self.previous != self.current
	}
}

impl Octave for Left {
	fn get(&self) -> u8 {
		self.octave
	}

	fn set(&mut self, direction: bool) {
		if direction {
			self.octave += 1;

			if self.octave > 9 {
				self.octave = 3;
			}
		}
		else {
			self.octave -= 1;

			if self.octave < 3 {
				self.octave = 9;
			}
		}
	}
}

impl Deref for Left {
	type Target = State;

	fn deref(&self) -> &Self::Target {
		&self.current
	}
}

fn led(state: &State) -> u8 {
	let button = state.button.unwrap();
	let level  = if state.trigger {
		match button {
			Button::A => 70,
			Button::B => 80,
			Button::C => 90,
			Button::D => 100,
		}
	}
	else {
		match button {
			Button::A => 30,
			Button::B => 40,
			Button::C => 50,
			Button::D => 60,
		}
	};

	if state.grip {
		level + 5
	}
	else {
		level
	}
}

fn build<'a, 'b>(mut builder: controller::Sound<'a, 'b>, state: &State, octave: u8) -> controller::Sound<'a, 'b> {
	let button = state.button.unwrap();

	builder = if state.trigger {
		match button {
			Button::A => builder.note(Note::G).octave(octave),
			Button::B => builder.note(Note::A).octave(octave),
			Button::C => builder.note(Note::B).octave(octave),
			Button::D => builder.note(Note::C).octave(octave + 1),
		}
	}
	else {
		match button {
			Button::A => builder.note(Note::C).octave(octave),
			Button::B => builder.note(Note::D).octave(octave),
			Button::C => builder.note(Note::E).octave(octave),
			Button::D => builder.note(Note::F).octave(octave),
		}
	};

	if state.grip {
		builder = builder.sharp();
	}

	builder
}

fn main() {
	let mut manager    = controller::Manager::new().unwrap();
	let mut controller = manager.open().unwrap();

	controller.led().off().unwrap();

	let mut left  = Left::default();
	let mut right = Right::default();

	loop {
		match controller.state(Duration::from_secs(0)).unwrap() {
			controller::State::Input { buttons, pad, .. } => {
				left.update(buttons, pad);
				right.update(buttons, pad);

				if left.has_update() {
					match left.mode {
						Some(Mode::Octave(octave)) =>
							Octave::set(&mut left, octave),

						_ => ()
					}

					if left.button.is_some() {
						build(controller.sound().left(), &left, left.octave).play().unwrap();
						controller.led().level(led(&left)).unwrap();
					}
					else {
						controller.sound().left().stop().unwrap();
						controller.led().off().unwrap();
					}
				}

				if right.has_update() {
					match right.mode {
						Some(Mode::Octave(octave)) =>
							Octave::set(&mut right, octave),

						_ => ()
					}

					if right.button.is_some() {
						build(controller.sound().right(), &right, right.octave).play().unwrap();
						controller.led().level(led(&right)).unwrap();
					}
					else {
						controller.sound().right().stop().unwrap();
						controller.led().off().unwrap();
					}
				}
			}

			_ => ()
		}
	}
}
