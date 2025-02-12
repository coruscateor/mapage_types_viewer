use gtk_estate::{adw::{glib::ExitCode, prelude::*, Application}, StateContainers}; //ClearStateContainersOnDrop,

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

mod all_or_not_supported_type_sub_contents;

pub use all_or_not_supported_type_sub_contents::*;

mod all_or_not_whatever_sub_contents;

pub use all_or_not_whatever_sub_contents::*;

mod tab_indenter;

pub use tab_indenter::*;

mod all_or_not_type_instance_sub_contents;

pub use all_or_not_type_instance_sub_contents::*;

mod whatever_shared;

pub use whatever_shared::*;

mod command_sub_contents;

pub use command_sub_contents::*;

mod supported_type_sub_contents;

pub use  supported_type_sub_contents::*;

mod type_instance_sub_contents;

pub use type_instance_sub_contents::*;

mod whatever_sub_contents;

pub use whatever_sub_contents::*;

mod optional_value_sub_contents;

pub use optional_value_sub_contents::*;

mod params_sub_contents;

pub use params_sub_contents::*;

mod command_shared;

pub use command_shared::*;

mod command_result_sub_contents;

pub use command_result_sub_contents::*;

mod command_error_sub_contents;

pub use command_error_sub_contents::*;

mod streamed_message_sub_contents;

pub use streamed_message_sub_contents::*;

fn main() -> ExitCode
{

    //println!("Start");

    //let _clear_scs = ClearStateContainersOnDrop::get();

    let app = Application::builder().application_id("org.mapage_types_viewer").build();
    
    ApplicationState::new(&app);

    app.run()

}
