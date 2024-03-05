#![deny(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::restriction,
    warnings,
    nonstandard_style,
    rust_2018_compatibility,
    unused
)]
#![allow(
    clippy::print_stdout,
    clippy::use_debug,
    clippy::missing_docs_in_private_items,
    clippy::absolute_paths,
)]
#![allow(clippy::blanket_clippy_restriction_lints)] //workaround clippy
#![allow(clippy::needless_return)]
// might want to deny later:
#![allow(clippy::default_numeric_fallback)] // might want to deny later!
#![allow(clippy::dbg_macro)]

use egui::ViewportCommand;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
#[allow(clippy::module_name_repetitions)] //FIXME: deny this!
pub struct TemplateApp {
    // Example stuff:
    label: String,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,

    cnt1: i32,
}

impl Default for TemplateApp {
    #[inline]
    fn default() -> Self {
        return Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            cnt1: 0,
        };
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    #[inline]
    #[must_use]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        //return Default::default() // not as clear, as per clippy
        //return TemplateApp::default() //works as well,but error: unnecessary structure name repetition
        return Self::default(); //works as well,but error: unnecessary structure name repetition

        //return self::TemplateApp::default() //works as well
    }
}

#[allow(clippy::missing_trait_methods)] //FIXME: deny this
impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    #[inline]
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    #[inline]
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        /*let Self {
            label,
            value,
            //mut cnt1, //goof 1of2 ; so this 'mut' makes it 'i32'(due to Copy trait) but w/o it it's '&mut i32' ; caught by clippy:    = note: `#[deny(clippy::pattern_type_mismatch)]` implied by `#[deny(clippy::restriction)]`

            cnt1, //goof_fixed 1of2 (works but clippy doesn't like it!)
        } = self; */
        let Self {
            ref mut label,
            ref mut value,
            ref mut cnt1,
        } = *self; // clippy likes it 1of2
                   //in the 'goof' case, we're mutating local copy of the original self.cnt1 when +/- in the ui_counter() function happens
                   //let Self { ref mut label, ref mut value, ref mut cnt1 } = *self;
                   //^ this works too, thanks Arnavion #rust liberachat
        egui::Window::new("Window").show(ctx, |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally choose either panels OR windows.");
            ui_counter(
                ui,
                //&mut cnt1 //goof 2of2
                //cnt1 //goof_fixed 2of2
                cnt1, //clippy likes it 2of2
            );
        });

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui2| {
                ui2.menu_button("File", |ui3| {
                    if ui3.button("Quit").clicked() {
                        //frame. close(); // XXX: Most commands in `eframe::Frame` has been replaced with `egui::ViewportCommand`, so So `frame.close()` becomes `ctx.send_viewport_cmd(ViewportCommand::Close)`, etc.
                        ctx.send_viewport_cmd(ViewportCommand::Close); // Close the window
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui1| {
                ui1.label("Write something: ");
                ui1.text_edit_singleline(label);
            });

            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));

            //https://doc.rust-lang.org/reference/expressions.html#expression-attributes
            //must place those 2 on the 'if' else they've no effect on the assingment!
            #[allow(clippy::float_arithmetic)]
            #[allow(clippy::arithmetic_side_effects)]
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui1| {
                ui1.horizontal(|ui2| {
                    ui2.spacing_mut().item_spacing.x = 0.0;
                    ui2.label("powered by ");
                    ui2.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui2.label(" and ");
                    ui2.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui2.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });
    }
}

//src: https://docs.rs/egui/latest/egui/#a-simple-example
fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
    // Put the buttons and label on the same row:
    ui.horizontal(|ui1| {
        // error: lint `clippy::integer_arithmetic` has been renamed to `clippy::arithmetic_side_effects`
        #[allow(clippy::arithmetic_side_effects)]
        if ui1.button("-").clicked() {
            *counter -= 1;
        }
        ui1.label(counter.to_string());
        // error: lint `clippy::integer_arithmetic` has been renamed to `clippy::arithmetic_side_effects`
        #[allow(clippy::arithmetic_side_effects)]
        if ui1.button("+").clicked() {
            *counter += 1;
        }
    });
}
