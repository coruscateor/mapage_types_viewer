use std::rc::Rc;

use std::any::Any;

use gtk_estate::adw::glib::Propagation;
use gtk_estate::corlib::{impl_as_any_ref, convert::AsAnyRef};

use gtk_estate::gtk4::prelude::GtkWindowExt;
use gtk_estate::{impl_application_state_container_traits, scs_set_application_state, WidgetStateContainer };

use gtk_estate::{adw::{prelude::ApplicationExt, Application}, AdwApplicationWindowState, ApplicationAdapter, ApplicationStateContainer, StateContainers, StoredApplicationObject, DynApplicationStateContainer};

use tokio::runtime::{Builder, Handle, Runtime};

use crate::WindowContentsState;

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

            Propagation::Proceed

        });

    }

}

impl_application_state_container_traits!();

