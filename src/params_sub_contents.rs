use std::{cell::Cell, ops::Deref, process::Command, rc::{Rc, Weak}, str::FromStr};

use gtk_estate::{adw::prelude::ButtonExt, gtk4::{prelude::{BoxExt, Cast, WidgetExt}, Button, CheckButton, ScrolledWindow, Text}, WidgetContainer};

use crate::{widgets::new_supported_type_strs_dropdown, AllOrNot, SupportedType, SupportedTypeSubContents, TypeInstance, TypeInstanceSubContents, WindowContentsState};

use corlib::{cell::RefCellStore, events::PubSingleSubEvent, impl_pub_single_sub_event_method, inc_dec::IncDecSelf, upgrading::try_up_rc, value::{HasOptionalValueGetter, HasValueGetter}};

use corlib::events::SingleSubEvent; 

use gtk_estate::gtk4::{Align, Box, DropDown, Label, Orientation, StringObject, Widget};

use gtk_estate::gtk4::glib::clone;

use crate::OptionalValueSubContents;

pub struct ParamsSubContents<P>
    where P: 'static
{

    params_contents_vec: RefCellStore<Vec<Rc<OptionalValueSubContents<TypeInstanceSubContents<P>>>>>,
    contents_box: Box,
    params_contents_box: Box,
    add_button: Button,
    remove_button: Button

}

impl<P> ParamsSubContents<P>
    where P: 'static
{

    pub fn new() -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        //

        let id_text_label = Label::builder().label("Params").halign(Align::Start).build();

        contents_box.append(&id_text_label);

        //

        let params_contents_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        let params_contents_box_sw = ScrolledWindow::builder().child(&params_contents_box).build();

        contents_box.append(&params_contents_box_sw);

        //

        let buttons_box = Box::builder().orientation(Orientation::Horizontal).spacing(2).build();

        let add_button = Button::builder().label("add").build();

        let remove_button = Button::builder().label("remove").build();

        buttons_box.append(&add_button);

        buttons_box.append(&remove_button);

        contents_box.append(&buttons_box);

        //

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                params_contents_vec: RefCellStore::new(Vec::new()),
                contents_box,
                params_contents_box,
                add_button,
                remove_button

            }
        
        });

        this.add_button.connect_clicked(clone!( #[strong] this, move |_button|
        {

            this.append_type_instance_contents();

        }));

        this.remove_button.connect_clicked(clone!( #[strong] this, move |_button|
        {

            this.remove_type_instance_contents();

        }));

        this

    }

    fn append_type_instance_contents(&self)
    {

        let ov_sc = self.params_contents_vec.borrow_mut(|mut state|
        {
            let sc = TypeInstanceSubContents::<P>::new();

            let ov_sc = OptionalValueSubContents::new(sc);

            state.push(ov_sc.clone());

            ov_sc

        });

        self.params_contents_box.append(ov_sc.widget_ref());

    }

    fn remove_type_instance_contents(&self)
    {

        //let self.params_contents_box.last_child();

        let poped = self.params_contents_vec.borrow_mut(|mut state|
        {

            state.pop()

            /* 
            if let Some(val) = state.pop()
            {
                
              return Some(val;

            }

            None
            */

            /*
            if !state.is_empty()
            {

                let state.end //[state.len() - 1]

            }
            */

        });

        if let Some(val) = poped
        {

            self.params_contents_box.remove(val.widget_ref());

        }

    }

    //impl_pub_single_sub_event_method!(on_supported_type_str_selected, P);

}

impl<P> WidgetContainer for ParamsSubContents<P>
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

impl<P> HasValueGetter for ParamsSubContents<P>
{

    type HasValueType = Result<Vec<Option<TypeInstance>>, String>;

    fn value(&self) -> Self::HasValueType
    {

        self.params_contents_vec.borrow(|state|
        {
            let mut vec_result = Vec::with_capacity(state.len());

            let mut index: usize = 0;

            for item in state.iter()
            {
    
                match item.value()
                {
    
                    Some(val) =>
                    {
    
                        match val
                        {
    
                            Ok(res) =>
                            {
    
                                vec_result.push(Some(res));
    
                            }
                            Err(err) =>
                            {
    
                                return Err(format!("Parameter error at index: {}, message: {}", index, err));
    
                            }
    
                        }
    
                    }
                    None =>
                    {
    
                        vec_result.push(None);
    
                    }
    
                }
    
                index.pp();
    
            }
    
            Ok(vec_result)

        })

    }

}




