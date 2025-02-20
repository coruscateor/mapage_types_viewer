use crate::{actors::MapageTypeActorOutputMessage, TabIndenter};

use super::{check_for_nulls, ClientOutputter};

use async_recursion::async_recursion;
use corlib::inc_dec::IncDecSelf;
use libsync::{crossbeam::mpmc::tokio::array_queue::Sender, BoundedSendError};
use serde_json::Value;


pub struct SerdeJsonProcessor
{

    client_outputter: ClientOutputter

}

impl SerdeJsonProcessor
{

    pub fn new(sender: Sender<MapageTypeActorOutputMessage>) -> Self
    {

        Self
        {
            
            client_outputter: ClientOutputter::new(sender)
        
        }

    }

    async fn send_serde_json_value_heading(&self) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        self.client_outputter.send_str("serde_json::Value:: ...\n\n\n\n").await

    }

    #[async_recursion]
    async fn send_serde_json_value_enum_string_parts<'a>(&'a self, value: &Value, tab_indenter: &TabIndenter<'a>) -> Result<(), BoundedSendError<MapageTypeActorOutputMessage>>
    {

        match value
        {

            serde_json::Value::Null =>
            {

                self.client_outputter.send_str("Null").await?;

            },
            serde_json::Value::Bool(val) =>
            {

                self.client_outputter.send_str("Bool(").await?;

                self.client_outputter.send_string(val.to_string()).await?;

                self.client_outputter.send_str(")").await?;

            }
            serde_json::Value::Number(number) =>
            {

                self.client_outputter.send_str("Number(").await?;

                self.client_outputter.send_string(number.to_string()).await?;

                self.client_outputter.send_str(")").await?;

            }
            serde_json::Value::String(string) =>
            {

                self.client_outputter.send_str("String(\"").await?;

                match check_for_nulls(string)
                {
                    Some(new_string) => self.client_outputter.send_string(new_string).await?,
                    None => self.client_outputter.send_string_clone(string).await?
                }

                self.client_outputter.send_str("\")").await?;

            }
            serde_json::Value::Array(vec) =>
            {

                self.client_outputter.send_str("Vec([\n\n").await?;

                let mut len = vec.len();

                let tab_indenter_elements = tab_indenter.next();

                for item in vec
                {

                    tab_indenter_elements.send_indentation().await?;

                    self.send_serde_json_value_enum_string_parts(item, &tab_indenter_elements).await?;

                    len.mm();

                    if len > 0
                    {

                        self.client_outputter.send_str(",\n\n").await?;

                    }
                    else
                    {

                        self.client_outputter.send_2_newlines().await?;
                        
                    }

                }

                tab_indenter.send_indentation().await?;

                self.client_outputter.send_str("])").await?;                     

            }
            serde_json::Value::Object(map) =>
            {

                self.client_outputter.send_str("Object({\n\n").await?;

                let mut len = map.len();

                let tab_indenter_fields = tab_indenter.next();

                for item in map
                {

                    tab_indenter_fields.send_indentation().await?;

                    self.client_outputter.send_str("\"").await?;

                    self.client_outputter.send_string_clone(item.0).await?;

                    self.client_outputter.send_str("\": ").await?;

                    self.send_serde_json_value_enum_string_parts(item.1, &tab_indenter_fields).await?;

                    len.mm();

                    if len > 0
                    {

                        self.client_outputter.send_str(",\n\n").await?;

                    }
                    else
                    {

                        self.client_outputter.send_2_newlines().await?;
                        
                    }

                }

                tab_indenter.send_indentation().await?;

                self.client_outputter.send_str("})").await?;

            }

        }

        Ok(())

    }

}