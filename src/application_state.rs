use std::rc::{Rc, Weak};

use std::any::Any;

use corlib::cell::RefCellStore;

use corlib::impl_weak_self_trait;
use corlib::inc_dec::IncDecSelf;

use corlib::WeakSelf;

use gtk_estate::adw::glib;

use gtk_estate::adw::glib::types::StaticType;

use gtk_estate::adw::glib::{clone, Propagation};

use gtk_estate::adw::ApplicationWindow;

use gtk_estate::corlib::{impl_as_any_ref, convert::AsAnyRef};

use gtk_estate::gtk::prelude::{GtkApplicationExt, GtkWindowExt, WidgetExt};

use gtk_estate::{scs_set_application_state, widget_upgrade_error_debug_println, WidgetStateContainer, WidgetUpgradeResult}; //RcApplicationAdapter, , WidgetStateContainer

//impl_application_state_container_traits,

use gtk_estate::{adw::{prelude::ApplicationExt, Application}, StateContainers}; //AdwApplicationWindowState, //ApplicationAdapter, ApplicationStateContainer, , StoredApplicationObject, DynApplicationStateContainer

use tokio::runtime::{Builder, Handle, Runtime};

use gtk_estate::gtk::Box;

use crate::WindowContentsState;

#[derive(Debug)]
pub struct ApplicationState
{

    app: Application,
    tokio_rt: Runtime,
    //application_adapter: RcApplicationAdapter<Application, ApplicationState>
    //weak_window_states: RefCellStore<Vec<Weak<WindowContentsState>>>
    weak_self: Weak<ApplicationState>

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
                //application_adapter: ApplicationAdapter::new(app, weak_self)
                //weak_window_states: RefCellStore::new(Vec::new())
                weak_self: weak_self.clone()

            }

        });

        //let ws = this.application_adapter.weak_parent();

        //this.application_adapter.application()

        app.connect_activate(clone!( #[weak] this, move |_app|
        {

            widget_upgrade_error_debug_println(this.new_window());

            //if let Some(this) = ws.upgrade()
            //{

                //this.new_window();
                
            //}

        }));

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

    pub fn new_window(&self) -> WidgetUpgradeResult
    {

        let adw_app_window_state = WindowContentsState::new(&self.app);

        let weak_window_state = adw_app_window_state.weak_self();

        /*
        self.weak_window_states.borrow_mut_with_param(weak_window_state, |mut state, weak_window_state|
        {

            state.push(weak_window_state);

        });
        */

        //let content = WindowContentsState::new(&self.app);

        /*
        let adw_app_window_state= AdwApplicationWindowState::builder_with_content_visible(|builder| {

            builder.application(&self.app) //&self.application_adapter.application())
            .default_width(1000)
            .default_height(1000)
            //.build()

        }, &content);
        */

        let app_window = adw_app_window_state.widget_adapter().widget()?;

        /*
        app_window.connect_close_request(|_window| {

            //window.destroy();

            let scs = StateContainers::get();

            let widget_state_ref = scs.widget_state_ref();

            //let _res = widget_state_ref.remove_by_widget_ref(window);
            println!("scs buckets_len: {}", widget_state_ref.buckets_len());

            println!("scs buckets_capacity: {}", widget_state_ref.buckets_capacity());

            println!("scs bucket_len ApplicationWindow {:#?}", widget_state_ref.bucket_len(&ApplicationWindow::static_type()));

            println!("scs bucket_capacity ApplicationWindow {:#?}", widget_state_ref.bucket_len(&ApplicationWindow::static_type()));
        
            //println!("scs bucket_len Box {:#?}", widget_state_ref.bucket_len(&Box::static_type()));

            //println!("scs bucket_capacity Box {:#?}", widget_state_ref.bucket_len(&Box::static_type()));

            //println!("scs after remove_by_widget_ref: {:?}\n\n", scs);

            Propagation::Proceed

        });
        */

        /*
        if let Some(parent) = app_window.parent()
        {

            println!("Adw::ApplicationWindow parent: {:?}\n\n", parent);

        }
        else
        {

            println!("Adw::ApplicationWindow no parent\n\n");
            
        }

        println!("In Adw::Application:\n\n");

        let app_windows = self.app.windows(); //self.application_adapter.application().windows();

        println!("Adw::Application Windows len: {:?}\n\n", app_windows.len());

        println!("app_windows.iter()\n\n");

        for item in app_windows.iter()
        {

            println!("Adw::ApplicationWindow:\n\n{:?}\n\n", item);

        }
        */

        /*
        println!("weak_window_states\n\n");

        self.weak_window_states.borrow(|state|
        {

            let mut number = 1;

            for item in state.iter()
            {
    
                println!("{}, Strong Count: {} Weak Count: {}", number, item.strong_count(), item.weak_count());

                number.pp();
    
            }

        });
        */
        
        Ok(())

    }

}

impl_weak_self_trait!(ApplicationState);

//impl_application_state_container_traits!();

impl Drop for ApplicationState
{

    fn drop(&mut self)
    {

        println!("Dropping ApplicationState")
       
    }

}

