use std::{cell::Cell, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::{glib::clone::Downgrade, prelude::{BoxExt, Cast, WidgetExt}}, gtk4::{Align, Box, DropDown, Label, Orientation, StringObject}};

use crate::{widgets::new_supported_type_strs_dropdown, AllOrNot, SupportedType, WindowContentsState};

use corlib::{events::SubUnSub, upgrading::try_up_rc};

use corlib::events::SingleSubEvent; 

use delegate::delegate;

pub struct SupportedTypeSubContents
{

    supported_type_strs_dropdown: DropDown,
    supported_type_box: Box,
    all_or_not_supported_type: Cell<AllOrNot<SupportedType>>, //all_or_not_supported_type_rc: Rc<Cell<AllOrNot<SupportedType>>>,
    supported_type_str_selected_event: SingleSubEvent<Self, WindowContentsState>

}

impl SupportedTypeSubContents
{

    pub fn new() -> Rc<Self>
    {

        let label = Label::builder().label("SupportedType").halign(Align::Start).build();

        //

        let supported_type_strs_dropdown = new_supported_type_strs_dropdown();

        supported_type_strs_dropdown.set_width_request(120);

        //

        let supported_type_strs_dropdown_box = Box::builder().orientation(Orientation::Horizontal).spacing(5).visible(true).build();

        supported_type_strs_dropdown_box.append(&supported_type_strs_dropdown);

        //

        let supported_type_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        supported_type_box.append(&label);

        supported_type_box.append(&supported_type_strs_dropdown_box);

        //let all_or_not_supported_type_rc = Rc::new(Cell::new(AllOrNot::All));

        //let all_or_not_supported_type_weak = all_or_not_supported_type_rc.downgrade();

        let this = Rc::new(Self
        {

            supported_type_strs_dropdown,
            supported_type_box,
            all_or_not_supported_type: Cell::new(AllOrNot::All),
            supported_type_str_selected_event: SingleSubEvent::new()

        });

        let weak = this.downgrade();

        this.supported_type_strs_dropdown.connect_selected_item_notify(move |supported_type_strs_dropdown|
        {

            try_up_rc(&weak, |this|
            {

                if let Some(item) = supported_type_strs_dropdown.selected_item()
                {

                    if let Some(item) = item.downcast_ref::<StringObject>()
                    {

                        let item_string = item.string();

                        if item_string == "*"
                        {

                            this.all_or_not_supported_type.set(AllOrNot::All);

                        }
                        else
                        {

                            let from_str_res = SupportedType::from_str(&item_string);

                            match from_str_res
                            {
    
                                Ok(res) =>
                                {

                                    this.all_or_not_supported_type.set(AllOrNot::NotAll(res));

                                    //n_supported_type_str_selected
    
                                }
                                Err(err) =>
                                {
    
                                    //parent.output_error(err);
    
                                    panic!("{}", err)

                                }
    
                            }
                            
                        }

                    }

                }

            });

        });

        this

    }

    pub fn widget_ref(&self) -> &Box
    {

        &self.supported_type_box

    }

    pub fn all_or_not_supported_type(&self) -> AllOrNot<SupportedType>
    {

        self.all_or_not_supported_type.get()

    }

    pub fn supported_type_str_selected_event_sub_un_sub<'a>(&'a self) -> SubUnSub<'a, Self, WindowContentsState>
    {

        self.supported_type_str_selected_event.get_sub_un_sub()

    }

    /*
    delegate!
    {

        to self.on_supported_type_str_selected
        {

            #[call(subscribe)]
            pub fn subscribe_to_supported_type_str_selected<F>(&self, parent: Weak<WindowContentsState>, func: F)
                where F: FnMut(&Self, Rc<WindowContentsState>) + 'static;

        }

    }
    */

    /*

        borrowed data escapes outside of method
        `self` escapes the method body hererustcClick for full compiler diagnostic
        supported_type_sub_contents.rs(71, 20): `self` is a reference that is only valid in the method body
        supported_type_sub_contents.rs(71, 20): let's call the lifetime of this reference `'1`
        borrowed data escapes outside of method
        argument requires that `'1` must outlive `'static`rustcClick for full compiler diagnostic
        supported_type_sub_contents.rs(71, 20): `self` is a reference that is only valid in the method body
        supported_type_sub_contents.rs(71, 20): let's call the lifetime of this reference `'1`

    */

    /*
    pub fn connect(&self, weak_parent_ref: &Weak<WindowContentsState>)
    {

        let weak_parent = weak_parent_ref.clone();

        self.supported_type_strs_dropdown.connect_selected_item_notify(move |supported_type_strs_dropdown|
        {

            if let Some(item) = supported_type_strs_dropdown.selected_item()
            {

                if let Some(item) = item.downcast_ref::<StringObject>()
                {

                    let item_string = item.string();

                    if item_string == "*"
                    {

                        self.all_or_not_supported_type_cell.set(AllOrNot::All);

                    }
                    else
                    {

                        let from_str_res = SupportedType::from_str(&item_string);

                        match from_str_res
                        {

                            Ok(res) =>
                            {

                                self.all_or_not_supported_type_cell.set(AllOrNot::NotAll(res));

                            }
                            Err(err) =>
                            {

                                panic!("{}", err)

                                //this.text_output.buffer().set_text(&err.to_string());

                                //parent.output_error(err);

                            }

                        }
                        
                    }

                }

            }

        });

    }
    */

    /*
    pub fn connect(&self, weak_parent_ref: &Weak<WindowContentsState>)
    {

        let weak_parent = weak_parent_ref.clone();

        self.supported_type_strs_dropdown.connect_selected_item_notify(move |supported_type_strs_dropdown|
        {

            try_up_rc(&weak_parent, |parent|
            {

                if let Some(item) = supported_type_strs_dropdown.selected_item()
                {

                    if let Some(item) = item.downcast_ref::<StringObject>()
                    {

                        let item_string = item.string();

                        if item_string == "*"
                        {

                            parent.set_all_or_not_supported_type(AllOrNot::All);

                            /*
                            parent.mut_state.borrow_mut(|mut state|
                            {

                                state.supported_type = AllOrNot::All;

                                parent.text_output.buffer().set_text("");

                            });
                            */

                        }
                        else
                        {

                            let from_str_res = SupportedType::from_str(&item_string);

                            match from_str_res
                            {
    
                                Ok(res) =>
                                {

                                    parent.set_all_or_not_supported_type(AllOrNot::NotAll(res));
    
                                    /*
                                    parent.mut_state.borrow_mut(|mut state|
                                    {
    
                                        state.supported_type = AllOrNot::NotAll(res);
    
                                        parent.text_output.buffer().set_text("");
    
                                    })
                                    */
    
                                }
                                Err(err) =>
                                {
    
                                    //this.text_output.buffer().set_text(&err.to_string());
    
                                    parent.output_error(err);
    
                                }
    
                            }
                            
                        }

                    }

                }

            });

        });

    }
    */

}


