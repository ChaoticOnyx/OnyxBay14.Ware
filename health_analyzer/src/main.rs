// Read more https://github.com/rust-embedded/riscv-rt/blob/master/src/lib.rs

#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use health_analyzer::{DamageType, DamageTypeIterator, HealthAnalyzer};
use heap::Heap;
use pci::PciBus;
use plic::Plic;
use riscv::register::{
    mcause::{Exception, Trap},
    sstatus::FS,
};
use riscv_rt::entry;
use sgl::{
    gpu::{
        Boundable, BoundableExt, Color, MutBoundable, MutPositionable, Point, Positionable, Rect,
        TextAlign,
    },
    Image, Sgl, Text,
};

macro_rules! include_asset {
    ($path:literal) => {
        include_bytes!(concat!("../assets/", $path))
    };
}

extern "C" {
    static _sheap: u8;
    static _heap_size: u8;
}

#[global_allocator]
static mut HEAP: Heap = Heap::empty();
static mut PLUS_IMAGE: Option<Image> = None;
static mut BIOHAZARD_IMAGE: Option<Image> = None;

const FONT_SIZE: f64 = 14.0;

fn draw_title_screen(label: &str) {
    let sgl = Sgl::mut_get().as_mut().unwrap();

    let mut label = Text::new_dynamic(label)
        .with_size(Some(FONT_SIZE))
        .with_align(TextAlign::Center)
        .with_position(Point::new(sgl.bounds().hcenter(), 0.0))
        .with_color(Some(Color::green()));

    let mut plus_image = unsafe { PLUS_IMAGE.clone().unwrap() };

    plus_image.translate_y(8.0);
    plus_image.translate_x(sgl.bounds().hcenter() - 120.0 / 2.0);

    sgl.fill_screen(Some(Color::black()));

    sgl.draw_image_rect(
        &plus_image,
        Rect::new_from_position(plus_image.position(), 120.0, 120.0),
    );

    label.translate_y(120.0 + FONT_SIZE * 2.0);
    sgl.draw_text(&label);

    sgl.flush();
}

fn draw_scan_report(analyzer: &HealthAnalyzer) {
    let sgl = Sgl::mut_get().as_mut().unwrap();
    let mut pos = Point::new(4.0, 4.0);
    let mut max_string_width = 0.0;

    sgl.fill_screen(Some(Color::black()));

    for damage_type in DamageTypeIterator::default() {
        let damage = analyzer.damage(damage_type);
        let damage_name = match damage_type {
            DamageType::Asphyxiation => "Асфиксация",
            DamageType::Bloodloss => "Кровопотеря",
            DamageType::Blunt => "Ударный урон",
            DamageType::Cellular => "Клеточный урон",
            DamageType::Caustic => "Разъедающий урон",
            DamageType::Cold => "Обморожение",
            DamageType::Heat => "Ожоги",
            DamageType::Piercing => "Колотые раны",
            DamageType::Poison => "Отравление",
            DamageType::Radiation => "Радиационный фон",
            DamageType::Shock => "Шок",
            DamageType::Slash => "Порезы",
        };

        let color;
        let text = if damage == 0.0 {
            color = Color::green();
            format!("{damage_name}...НЕТ")
        } else {
            if damage > 0.0 {
                color = Color::red();
            } else {
                color = Color::green();
            }

            format!("{damage_name}: {damage:.1}")
        };

        let text = Text::new_dynamic(&text)
            .with_color(Some(color))
            .with_position(pos)
            .with_size(Some(FONT_SIZE));
        let text_size = sgl.measure_text_bounds(&text);

        max_string_width = f64::max(max_string_width, text_size.width());
        sgl.draw_text(&text);

        pos.y += FONT_SIZE;

        if pos.y > sgl.bounds().height() {
            pos.y = 4.0;
            pos.x += max_string_width;
        }
    }

    if analyzer.has_disease() {
        let biohazard =
            unsafe { BIOHAZARD_IMAGE.clone().unwrap() }.with_bounds(Rect::new_from_position(
                Point::new(
                    sgl.bounds().width() - 160.0 - 25.0,
                    sgl.bounds().vcenter() - 130.0 / 2.0 - 15.0,
                ),
                160.0,
                130.0,
            ));

        let mut text_position = Point::zero();
        text_position.x = biohazard.bounds().hcenter();
        text_position.y = biohazard.position().y + biohazard.bounds().height() + FONT_SIZE;

        let disease_text = Text::new_dynamic("** ОБНАРУЖЕН ВИРУС **")
            .with_color(Some(Color::red()))
            .with_align(TextAlign::Center)
            .with_position(text_position)
            .with_size(Some(FONT_SIZE));

        sgl.draw_image_rect(&biohazard, biohazard.bounds().clone());
        sgl.draw_text(&disease_text);
    }

    sgl.flush();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    bsod::bsod_panic(Sgl::mut_get().as_mut().unwrap(), info);
}

#[entry]
unsafe fn main() -> ! {
    riscv::register::mstatus::set_fs(FS::Initial);

    HEAP.init(
        &_sheap as *const u8 as *mut u8,
        &_heap_size as *const u8 as usize,
    );

    Sgl::default().init();
    let sgl = Sgl::mut_get().as_mut().unwrap();

    PLUS_IMAGE = Some(sgl.create_image(include_asset!("plus.bitmap"), 281, 282));
    BIOHAZARD_IMAGE = Some(sgl.create_image(include_asset!("biohazard.bitmap"), 431, 349));

    draw_title_screen("Ожидание сканирования");

    let pci = PciBus::default();
    let mut plic = Plic::default();
    let analyzer = pci
        .find_by_id(health_analyzer::DEVICE_ID)
        .map(HealthAnalyzer::from)
        .unwrap();

    plic.set_enabled(analyzer.device.irq_pin, true);
    plic.set_priority(analyzer.device.irq_pin, u8::MAX);

    riscv::interrupt::enable();
    riscv::register::mie::set_mext();

    loop {
        riscv::asm::wfi();
    }
}

#[export_name = "MachineExternal"]
unsafe fn external_interrupt() {
    let mut plic = Plic::default();
    let pci = PciBus::default();
    let analyzer = pci
        .find_by_id(health_analyzer::DEVICE_ID)
        .map(HealthAnalyzer::from)
        .unwrap();

    while let Some(irq) = plic.pending_irq() {
        plic.claim(irq);

        if irq == analyzer.device.irq_pin {
            draw_title_screen("Анализ...");
            draw_scan_report(&analyzer);
        }
    }
}

#[export_name = "ExceptionHandler"]
fn exception(_trap_frame: &riscv_rt::TrapFrame) -> ! {
    let Trap::Exception(exception) = riscv::register::mcause::read().cause() else {
        unreachable!()
    };

    match exception {
        Exception::InstructionMisaligned => panic!("Instruction misaligned"),
        Exception::InstructionFault => panic!("Instruction fault"),
        Exception::IllegalInstruction => panic!("Illegal instruction"),
        Exception::Breakpoint => panic!("Breakpoint"),
        Exception::LoadMisaligned => panic!("Load misaligned"),
        Exception::LoadFault => panic!("Load fault"),
        Exception::StoreMisaligned => panic!("Store misaligned"),
        Exception::StoreFault => panic!("Store fault"),
        Exception::UserEnvCall => panic!("User EnvCall"),
        Exception::SupervisorEnvCall => panic!("Supervisor EnvCall"),
        Exception::MachineEnvCall => panic!("Machine EnvCall"),
        Exception::InstructionPageFault => panic!("Instruction page fault"),
        Exception::LoadPageFault => panic!("Load page fault"),
        Exception::StorePageFault => panic!("Store page fault"),
        Exception::Unknown => panic!("Unknown"),
    }
}
