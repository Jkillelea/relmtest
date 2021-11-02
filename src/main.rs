/*
 * Copyright (c) 2020 Boucher, Antoni <bouanto@zoho.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of
 * this software and associated documentation files (the "Software"), to deal in
 * the Software without restriction, including without limitation the rights to
 * use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
 * the Software, and to permit persons to whom the Software is furnished to do so,
 * subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
 * FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
 * COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
 * IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

use gtk::{
    prelude::BuilderExtManual,
    Builder,
    Button,
    Inhibit,
    Label,
    Window,
    prelude::ButtonExt,
    prelude::LabelExt,
    prelude::WidgetExt,
};

use relm_derive::Msg;

use relm::{connect, Relm, Update, Widget};

struct Model {
    counter: i32,
}

#[derive(Msg)]
enum Msg {
    Decrement,
    Increment,
    Half,
    Quit,
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
    counter_label: Label,
    minus_button: Button,
    plus_button: Button,
    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, event: Msg) {
        let label = &self.widgets.counter_label;

        match event {
            Msg::Decrement => {
                // label.set_text(&self.model.counter.to_string());
                self.model.counter -= 1;
            },
            Msg::Increment => {
                self.model.counter += 1;
                // label.set_text(&self.model.counter.to_string());
            },
            Msg::Half => {
                self.model.counter = self.model.counter / 2;
                // label.set_text(&self.model.counter.to_string());
            },
            Msg::Quit => gtk::main_quit(),
        }

        label.set_text(&self.model.counter.to_string());
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let glade_src = include_str!("../data/main_window.glade");
        let builder = Builder::from_string(glade_src);

        let window: Window = builder.object("main_window").unwrap();

        let counter_label: Label  = builder.object("label").unwrap();
        counter_label.set_text("0");

        let plus_button:   Button = builder.object("inc_button").unwrap();
        let minus_button:  Button = builder.object("dec_button").unwrap();
        let half_button:   Button = builder.object("half_button").unwrap();
        let quit_button:   Button = builder.object("quit_button").unwrap();

        connect!(relm, plus_button,  connect_clicked(_), Msg::Increment);
        connect!(relm, minus_button, connect_clicked(_), Msg::Decrement);
        connect!(relm, half_button,  connect_clicked(_), Msg::Half);
        connect!(relm, quit_button,  connect_clicked(_), Msg::Quit);

        connect!(relm, window,
                 connect_delete_event(_, _),
                 return (Some(Msg::Quit), Inhibit(false))
        );

        window.show_all();

        Win {
            model: model,

            widgets: Widgets {
                counter_label: counter_label,
                minus_button: minus_button,
                plus_button: plus_button,
                window: window,
            },
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

