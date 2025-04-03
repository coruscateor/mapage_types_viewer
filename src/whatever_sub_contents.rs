use std::{cell::Cell, fmt::Display, ops::Deref, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::{clone::Downgrade, property::PropertyGet}, prelude::{BoxExt, Cast, EditableExt, IsA, TextBufferExt, TextViewExt, WidgetExt}}, gtk::{Align, Box, DropDown, Label, Orientation, ScrolledWindow, StringObject, Text, TextView, Widget}, impl_contents_box_ref, WidgetContainer};

use crate::{widgets::{new_whatever_strs_dropdown, new_whatever_strs_no_all_dropdown}, AllOrNot, WindowContentsState};

use corlib::{cell::RefCellStore, events::{PubSingleSubEvent, SingleSubArgsEvent}, impl_pub_single_sub_args_event_method, impl_pub_single_sub_event_method, inc_dec::IncDecSelf, upgrading::try_up_rc, value::HasValueGetter};

use corlib::events::{SingleSubEvent, PubSingleSubArgsEvent}; 

use gtk_estate::helpers::text_view::get_text_view_string;

use gtk_estate::gtk::glib;

use gtk_estate::gtk::glib::clone;

use crate::{try_set_specific_whatever, parse_error_at_index, parse_array};

use mapage_lib::Whatever;

pub struct WhateverSubContents<P>
    where P: 'static
{

    whatever_strs_dropdown: DropDown,
    contents_box: Box,
    whatever_result: RefCellStore<Result<Whatever, String>>,
    on_whatever_str_selected: SingleSubEvent<Self, WindowContentsState>,
    value_input: TextView,
    on_value_input_parse_error: SingleSubArgsEvent<Self, String, WindowContentsState>,
    detected_whatever_variant: TextView

}

impl<P> WhateverSubContents<P>
    where P: 'static
{

    pub fn new() -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(6).visible(true).build();

        //

        let label = Label::builder().label("Whatever").halign(Align::Start).build();

        contents_box.append(&label);

        //

        let whatever_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        let whatever_strs_dropdown = new_whatever_strs_no_all_dropdown();

        whatever_strs_dropdown.set_width_request(120);

        whatever_strs_dropdown_box.append(&whatever_strs_dropdown);

        contents_box.append(&whatever_strs_dropdown_box);

        //

        let value_input_label = Label::builder().label("Value Input").halign(Align::Start).build();

        contents_box.append(&value_input_label);

        //

        let value_input = TextView::builder().accepts_tab(false).build();

        let value_input_sw = ScrolledWindow::builder().child(&value_input).build();

        contents_box.append(&value_input_sw);

        //

        let detected_whatever_variant_label = Label::builder().label("Detected Variant Or Variants").halign(Align::Start).build();

        contents_box.append(&detected_whatever_variant_label);

        //

        //What is in all_or_not_whatever?

        let detected_whatever_variant = TextView::builder().editable(false).build(); //.text("All Variants")

        let detected_whatever_variant_sw = ScrolledWindow::builder().child(&detected_whatever_variant).build();

        contents_box.append(&detected_whatever_variant_sw);

        detected_whatever_variant.buffer().set_text("All Variants");

        //
    
        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                whatever_strs_dropdown,
                contents_box,
                whatever_result: RefCellStore::new(Ok(Whatever::default())),
                on_whatever_str_selected: SingleSubEvent::new(weak_self),
                value_input,
                on_value_input_parse_error: SingleSubArgsEvent::new(weak_self),
                detected_whatever_variant

            }
        
        });

        //Try set the whatever variant, or all, when is a String is selected.

        this.whatever_strs_dropdown.connect_selected_item_notify(clone!( #[weak] this, move |whatever_strs_dropdown|
        {

            if let Some(item) = whatever_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    this.try_set_whatever(&item_string);
                    
                }

            }

        }));

        this.value_input.connect_move_focus(clone!( #[weak] this, move |_value_input, _|
        {

            if let Some(item) = this.whatever_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    this.try_set_whatever(&item_string);
                    
                }

            }

        }));
        
        this

    }

    impl_contents_box_ref!();

    impl_pub_single_sub_event_method!(on_whatever_str_selected, WindowContentsState);

    impl_pub_single_sub_args_event_method!(on_value_input_parse_error, String, WindowContentsState);

    /*
    pub fn widget_ref(&self) -> &Box
    {

        &self.contents_box

    }

    pub fn whatever_result(&self) -> Result<Whatever, String>
    {

        self.all_or_not_whatever.get()

    }
    */

    fn try_set_whatever(&self, variant_str: &str)
    {

        let buffer = self.value_input.buffer();

        let start = buffer.start_iter();

        let end = buffer.end_iter();

        let buffer_text = buffer.text(&start, &end, false);

        let value_input_str = buffer_text.as_str();

        let the_res = try_set_specific_whatever(variant_str, value_input_str);

        match the_res
        {

            Ok((whatever, variant_string)) =>
            {

                self.whatever_result.set(Ok(whatever));

                self.detected_whatever_variant.buffer().set_text(&variant_string);

                self.on_whatever_str_selected.raise();

            }
            Err(error_message) =>
            {

                self.detected_whatever_variant.buffer().set_text(&error_message);

                self.whatever_result.set(Err(error_message));

                self.whatever_result.borrow(|store|
                {

                    if let Err(message) = &*store
                    {

                        self.on_value_input_parse_error.raise(message);

                    }

                })

            }

        }

    }

}

impl<P> WidgetContainer for WhateverSubContents<P>
{

    fn widget(&self) -> Widget
    {

        self.contents_box.upcast_ref::<Widget>().clone()
        
    }

    fn widget_ref(&self) -> &Widget
    {

        self.contents_box.upcast_ref::<Widget>()
        
    }

}

impl<P> HasValueGetter for WhateverSubContents<P>
{

    type HasValueType = Result<Whatever, String>;

    fn value(&self) -> Self::HasValueType
    {

        self.whatever_result.get()

    }

}