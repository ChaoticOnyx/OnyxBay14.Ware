use alloc::collections::VecDeque;
use hid::{
    keyboard::{Keyboard, KeyboardKey},
    KeyState,
};
use pci::PciBus;
use plic::Plic;

static mut EVENTS: Option<EventsQueue> = None;
static mut IO: Option<Io> = None;

type EventsQueue = VecDeque<IoEvent>;

const MAX_EVENTS: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IoEvent {
    Keyboard(KeyboardEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyboardEvent {
    pub key: KeyboardKey,
    pub state: KeyState,
}

pub struct Io {
    keyboard: Keyboard,
}

impl Io {
    pub fn init(mut self) {
        unsafe {
            let mut plic = Plic::default();
            self.keyboard.set_events(true);

            plic.set_threshold(0);
            plic.set_enabled(self.keyboard.device.irq_pin, true);
            plic.set_priority(self.keyboard.device.irq_pin, u8::MAX);

            riscv::interrupt::enable();
            riscv::register::mie::set_mext();

            Self::mut_io().replace(self);
            Self::mut_queue().replace(VecDeque::new());
        }
    }

    pub fn poll(&mut self) -> Option<IoEvent> {
        if let Some(queue) = Self::mut_queue() {
            return queue.pop_front();
        }

        None
    }

    pub fn poll_block(&mut self) -> IoEvent {
        loop {
            if let Some(ev) = self.poll() {
                return ev;
            }

            unsafe {
                riscv::asm::wfi();
            }
        }
    }

    pub fn keyboard(&self) -> &Keyboard {
        &self.keyboard
    }

    pub fn mut_queue() -> &'static mut Option<EventsQueue> {
        unsafe { &mut EVENTS }
    }

    pub fn queue() -> &'static Option<EventsQueue> {
        unsafe { &EVENTS }
    }

    pub fn mut_io() -> &'static mut Option<Io> {
        unsafe { &mut IO }
    }

    pub fn io() -> &'static Option<Io> {
        unsafe { &IO }
    }
}

impl Default for Io {
    fn default() -> Self {
        let pci = PciBus::default();
        let keyboard = pci
            .find_by_id(hid::keyboard::DEVICE_ID)
            .map(Keyboard::from)
            .unwrap();

        Self { keyboard }
    }
}

unsafe fn handle_keyboard_input(io: &mut Io) {
    let keyboard = &mut io.keyboard;
    let key = keyboard.last_changed_key();
    let key_state = keyboard.key_state(key);

    if let Some(queue) = Io::mut_queue() {
        if queue.len() >= MAX_EVENTS {
            return;
        }

        queue.push_front(IoEvent::Keyboard(KeyboardEvent {
            key,
            state: key_state,
        }))
    }
}

#[export_name = "MachineExternal"]
unsafe fn machine_external_handler() {
    let mut plic = Plic::default();

    while let Some(irq) = plic.pending_irq() {
        if let Some(io) = Io::mut_io() {
            let keyboard = &mut io.keyboard;

            if irq == keyboard.device.irq_pin {
                handle_keyboard_input(io);
            }
        }

        plic.claim(irq);
    }
}
