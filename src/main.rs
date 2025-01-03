use gtk_estate::{adw::{prelude::*, Application}, StateContainers};

mod application_state;

pub use application_state::*;

mod window_contents_state;

pub use window_contents_state::*;

mod streamed_message_etc;

pub use streamed_message_etc::*;

mod widgets;

mod actors;

mod all_or_not;

pub use all_or_not::*;

mod supported_type_sub_contents;

pub use supported_type_sub_contents::*;

fn main()
{

    let app = Application::builder().application_id("org.mapage_types_viewer").build();
    
    //Initaslise the StateContainers object.

    //StateContainers::init();

    ApplicationState::new(&app);

    app.run();

}
