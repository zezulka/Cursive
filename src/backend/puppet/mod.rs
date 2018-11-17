//! Puppet backend
use std::thread;

use std::collections::LinkedList;
use std::time::{Duration, Instant};
use std::cell::{Cell, RefCell};

use crossbeam_channel::{self, Receiver, Sender};

use backend;
use event::Event;
use theme;
use vec::Vec2;
use enumset::EnumSet;

pub mod observed;
use self::observed::ObservedFrame;

///
pub struct Backend {
    size : Vec2,
    inner_sender: Sender<Option<Event>>,
    inner_receiver: Receiver<Option<Event>>,
    frames : LinkedList<ObservedFrame>,
    current_frame : ObservedFrame,
    current_color : Cell<theme::ColorPair>,
    current_effect : RefCell<EnumSet<theme::Effect>>
}

impl Backend {
    ///
    pub fn init() -> Box<backend::Backend>
        where
            Self: Sized,
    {
        let (inner_sender, inner_receiver) = crossbeam_channel::bounded(1);
        let size = Vec2::new(100, 100); //TODO something less random
        let color_pair = theme::ColorPair {
            front: theme::Color::TerminalDefault,
            back : theme::Color::Dark(theme::BaseColor::Black)
        };

        Box::new(Backend {
            size : size,
            inner_sender : inner_sender,
            inner_receiver : inner_receiver,
            frames : LinkedList::new(),
            current_frame : ObservedFrame::new(size),
            current_color : Cell::new(color_pair),
            current_effect : RefCell::new(EnumSet::new())
        })
    }
}

impl backend::Backend for Backend {
    fn finish(&mut self) {}

    fn refresh(&mut self) {}

    fn has_colors(&self) -> bool {
        true
    }

    fn screen_size(&self) -> Vec2 {
        self.size
    }

    fn prepare_input(&mut self, _input_request: backend::InputRequest) {
        self.inner_sender.send(Some(Event::Exit));
    }

    fn start_input_thread(
        &mut self, event_sink: Sender<Option<Event>>,
        input_requests: Receiver<backend::InputRequest>,
    ) {
        let receiver = self.inner_receiver.clone();

        thread::spawn(move || {
            for _ in input_requests {
                match receiver.recv() {
                    None => return,
                    Some(event) => event_sink.send(event),
                }
            }
        });
    }

    fn print_at(&self, _: Vec2, _: &str) {}

    fn clear(&self, _: theme::Color) {}

    // This sets the Colours and returns the previous colours
    // to allow you to set them back when you're done.
    fn set_color(&self, colors: theme::ColorPair) -> theme::ColorPair {
        let old_colors = self.current_color.get();
        self.current_color.set(colors);
        old_colors
    }

    fn set_effect(&self, effect: theme::Effect) {
        self.current_effect.borrow_mut().insert(effect);
    }

    fn unset_effect(&self, effect: theme::Effect) {
        self.current_effect.borrow_mut().remove(effect);
    }
}