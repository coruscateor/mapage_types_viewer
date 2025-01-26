use std::marker::PhantomData;
use std::rc::Rc;

use corlib::value::HasOptionalValueGetter;
use gtk_estate::adw::prelude::{BoxExt, Cast};

use gtk_estate::gtk4::{Box, CheckButton, Orientation, Widget};

use gtk_estate::gtk4::prelude::{CheckButtonExt, WidgetExt};

use gtk_estate::WidgetContainer;

use gtk_estate::gtk4::glib::clone;

use gtk_estate::corlib::value::HasValueGetter;

pub struct OptionalValueSubContents<T>
    where T: WidgetContainer + HasValueGetter + 'static
{

    check_button: CheckButton,
    contained_sub_contents: Rc<T>,
    contents_box: Box,
    //phantom_data: PhantomData<R>

}

impl<T> OptionalValueSubContents<T>
    where T: WidgetContainer + HasValueGetter + 'static
{

    pub fn new(contained_sub_contents: Rc<T>) -> Rc<Self>
    {

        let contents_box = Box::builder().orientation(Orientation::Vertical).spacing(4).visible(true).build();

        //

        let check_button = CheckButton::builder().active(true).label("Is Some(...)").build();
        
        contents_box.append(&check_button);

        //

        contents_box.append(contained_sub_contents.widget_ref());

        //

        let this = Rc::new_cyclic(|_weak_self|
        {

            Self
            {

                check_button,
                contents_box,
                contained_sub_contents,
                //phantom_data: PhantomData::default()
                
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

impl<T> WidgetContainer for OptionalValueSubContents<T>
    where T: WidgetContainer + HasValueGetter + 'static
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

impl<T> HasOptionalValueGetter for OptionalValueSubContents<T>
    where T: WidgetContainer + HasValueGetter + 'static
{

    type HasValueType = T::HasValueType;

    fn value(&self) -> Option<Self::HasValueType>
    {

        if self.check_button.is_active()
        {

            return Some(self.contained_sub_contents.value());

        }

        None

    }

}

/*
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
*/