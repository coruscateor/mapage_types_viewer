use std::rc::Rc;

use std::any::Any;

use gtk_estate::adw::glib::types::StaticType;
use gtk_estate::adw::glib::Propagation;
use gtk_estate::adw::ApplicationWindow;
use gtk_estate::corlib::{impl_as_any_ref, convert::AsAnyRef};

use gtk_estate::gtk4::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};
use gtk_estate::{impl_application_state_container_traits, scs_set_application_state, WidgetStateContainer };

use gtk_estate::{adw::{prelude::ApplicationExt, Application}, AdwApplicationWindowState, ApplicationAdapter, ApplicationStateContainer, StateContainers, StoredApplicationObject, DynApplicationStateContainer};

use tokio::runtime::{Builder, Handle, Runtime};

use gtk_estate::gtk4::Box;

use crate::WindowContentsState;

#[derive(Debug)]
pub struct ApplicationState
{

    app: Application,
    tokio_rt: Runtime,
    application_adapter: Rc<ApplicationAdapter<Application, ApplicationState>>

}

impl ApplicationState
{

    pub fn new(app: &Application) -> Rc<Self>
    {

        let tokio_rt = Builder::new_multi_thread().enable_all().build().expect("Tokio Runtime construction failed");

        let this = Rc::new_cyclic(|weak_self|
        {
                
            Self
            {

                app: app.clone(),
                tokio_rt,
                application_adapter: ApplicationAdapter::new(app, weak_self)


            }

        });

        let ws = this.application_adapter.weak_parent();

        this.app.connect_activate(move |_app|
        {

            if let Some(this) = ws.upgrade()
            {

                this.new_window();
                
            }

        });

        scs_set_application_state!(this);

        this

    }


    pub fn tokio_rt_handle(&self) -> Handle
    {

        self.tokio_rt.handle().clone()

    }

    pub fn tokio_rt_handle_ref(&self) -> &Handle
    {

        self.tokio_rt.handle()

    }

    pub fn new_window(&self)
    {

        let content = WindowContentsState::new();

        let adw_app_window_state= AdwApplicationWindowState::builder_with_content_visible(|builder| {

            builder.application(&self.app)
            .default_width(1000)
            .default_height(1000)
            .build()

        }, &content);

        let app_window = adw_app_window_state.widget_adapter().widget();

        app_window.connect_close_request(|window| {

            //window.destroy();

            let scs = StateContainers::get();

            let _res = scs.remove_by_widget_ref(window);

            println!("scs buckets_len: {}", scs.buckets_len());

            println!("scs buckets_capacity: {}", scs.buckets_capacity());

            println!("scs bucket_len ApplicationWindow {:#?}", scs.bucket_len(&ApplicationWindow::static_type()));

            println!("scs bucket_capacity ApplicationWindow {:#?}", scs.bucket_len(&ApplicationWindow::static_type()));
        
            println!("scs bucket_len Box {:#?}", scs.bucket_len(&Box::static_type()));

            println!("scs bucket_capacity Box {:#?}", scs.bucket_len(&Box::static_type()));

            //println!("scs after remove_by_widget_ref: {:?}\n\n", scs);

            Propagation::Proceed

        });

        if let Some(parent) = app_window.parent()
        {

            println!("Adw::ApplicationWindow parent: {:?}\n\n", parent);

        }
        else
        {

            println!("Adw::ApplicationWindow no parent\n\n");
            
        }

        println!("In Adw::Application:\n\n");

        let app_windows = self.app.windows();

        println!("Adw::Application Windows len: {:?}\n\n", app_windows.len());

        for item in app_windows.iter()
        {

            println!("Adw::ApplicationWindow:\n\n{:?}\n\n", item);

        }

    }

}

impl_application_state_container_traits!();

impl Drop for ApplicationState
{

    fn drop(&mut self)
    {

        println!("Dropping ApplicationState")
       
    }

}

