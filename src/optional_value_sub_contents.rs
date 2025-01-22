use std::marker::PhantomData;
use std::rc::Rc;

use corlib::value::HasOptionalValueGetter;
use gtk_estate::adw::prelude::BoxExt;

use gtk_estate::gtk4::{CheckButton, Orientation, Box};

use gtk_estate::gtk4::prelude::{CheckButtonExt, WidgetExt};

use gtk_estate::WidgetContainer;

use gtk_estate::gtk4::glib::clone;

use gtk_estate::corlib::value::HasValueGetter;

pub struct OptionalValueSubContents<T, R>
    where T: WidgetContainer + HasValueGetter<R> + 'static,
          R: 'static
{

    check_button: CheckButton,
    contained_sub_contents: Rc<T>,
    optional_value_box: Box,
    phantom_data: PhantomData<R>

}

impl<T, R> OptionalValueSubContents<T, R>
    where T: WidgetContainer + HasValueGetter<R> + 'static,
          R: 'static
{

    pub fn new(contained_sub_contents: Rc<T>) -> Rc<Self>
    {

        let optional_value_box = Box::builder().orientation(Orientation::Vertical).spacing(2).visible(true).build();

        //

        let check_button = CheckButton::builder().active(true).label("Is Some(...)").build();
        
        optional_value_box.append(&check_button);

        //

        optional_value_box.append(contained_sub_contents.widget_ref());

        //

        let this = Rc::new_cyclic(|_weak_self|
        {

            Self
            {

                check_button,
                optional_value_box,
                contained_sub_contents,
                phantom_data: PhantomData::default()
                
            }

        });

        this.check_button.connect_toggled(clone!( #[strong] this, move |check_button|
        {

            this.contained_sub_contents.widget_ref().set_sensitive(check_button.is_active());

        }));

        this

    }

    //pub fn contained_sub_contents: Rc<T>

}

impl<T, R> HasOptionalValueGetter<R> for OptionalValueSubContents<T, R>
    where T: WidgetContainer + HasValueGetter<R> + 'static,
          R: 'static
{

    fn value(&self) -> Option<R>
    {

        if self.check_button.is_active()
        {

            return Some(self.contained_sub_contents.value());

        }

        None

    }

}