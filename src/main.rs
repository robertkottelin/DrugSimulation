use gtk::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

struct Cell {
    receptors: Vec<Receptor>,
    proteins: Vec<Protein>,
    dna: DNA,
    metabolites: Vec<Metabolite>,
}

// Other structs and impl blocks for Cell, Receptor, Protein, DNA, and
// Metabolite as shown in the previous example

fn main() {
    // Initialize GTK and create a new window
    gtk::init().expect("Failed to initialize GTK.");
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Cell Simulation");
    window.set_default_size(400, 400);

    // Set up the layout and widgets
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let label = gtk::Label::new(None);
    let button = gtk::Button::new_with_label("Trigger event");

    vbox.pack_start(&label, true, true, 0);
    vbox.pack_start(&button, false, false, 0);
    window.add(&vbox);

    // Create the cell and set up the label to display its state
    let cell = Rc::new(RefCell::new(Cell::new()));
    let cell_clone = cell.clone();
    label.set_text(&cell_clone.borrow().to_string());

    // Set up the button to trigger an event in the cell
    let cell_clone = cell.clone();
    button.connect_clicked(move |_| {
        let mut cell = cell_clone.borrow_mut();
        cell.trigger_event();
        label.set_text(&cell.to_string());
    });

    // Show the window and start the GTK main loop
    window.show_all();
    gtk::main();
}
