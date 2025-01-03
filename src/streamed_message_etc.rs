use std::{error::Error, fmt::Display};

use corlib::text::SendableText;

use serde::{Deserialize, Serialize};

use strum_macros::{AsRefStr, EnumCount, EnumIter, EnumString, FromRepr, IntoStaticStr}; //Display, 

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, EnumString, FromRepr, EnumIter, AsRefStr, EnumCount, IntoStaticStr)]
pub enum SupportedType
{

    #[default]
    Bool,
    Char,

    F32,
    F64,
    I8,
    I16,
    I32,
    I64,

    I128,
    //Isize,
    U8,
    U16,
    U32,
    U64,

    U128,
    //Usize,

    //Collections

    String,
    Whatever,

    //Vecs

    VecBool,
    //VecChar,

    VecF32,
    VecF64,
    VecI8,
    VecI16,
    VecI32,
    VecI64,

    VecI128,
    //VecIsize,
    VecU8,
    VecU16,
    VecU32,
    VecU64,

    VecU128,
    //VecUsize,

    //VecString,
    //VecWhatever

}

#[derive(Debug, Serialize, Deserialize, Clone, EnumString, FromRepr, EnumIter, AsRefStr, EnumCount, IntoStaticStr)]
pub enum Whatever
{

    Bool(bool),
    Char(char),

    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    I128(i128),
    //Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    U128(u128),
    //USize(usize),

    //Collections

    String(String),

    //Vecs

    VecBool(Vec<bool>),
    //VecChar(Vec<char>),

    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
    VecI8(Vec<i8>),
    VecI16(Vec<i16>),
    VecI32(Vec<i32>),
    VecI64(Vec<i64>),

    VecI128(Vec<i128>),
    //VecISize(Vec<isize>),
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),

    VecU128(Vec<u128>),
    //VecUSize(Vec<usize>),

    //VecString(Vec<String>)

}

impl Default for Whatever
{

    fn default() -> Self
    {
        
        Self::Bool(bool::default())

    }

}

#[derive(Debug, Serialize, Deserialize, EnumString, FromRepr, EnumIter, AsRefStr, EnumCount, IntoStaticStr)]
pub enum TypeInstance
{

    Bool(bool),
    Char(char),

    F32(f32),
    F64(f64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),

    I128(i128),
    //Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),

    U128(u128),
    //Usize(usize),

    //Collections

    String(String),

    Whatever(Whatever),

    //Vecs

    VecBool(Vec<bool>),
    //VecChar(Vec<char>),

    VecF32(Vec<f32>),
    VecF64(Vec<f64>),
    VecI8(Vec<i8>),
    VecI16(Vec<i16>),
    VecI32(Vec<i32>),
    VecI64(Vec<i64>),

    VecI128(Vec<i128>),
    //VecISize(Vec<isize>),
    VecU8(Vec<u8>),
    VecU16(Vec<u16>),
    VecU32(Vec<u32>),
    VecU64(Vec<u64>),

    VecU128(Vec<u128>),
    //VecUSize(Vec<usize>),

    //VecString(Vec<String>),
    //VecWhatever(Vec<Whatever>),
    //VecOptionWhatever(Vec<Option<Whatever>>),

}

impl Default for TypeInstance
{

    fn default() -> Self
    {
        
        Self::Bool(bool::default())

    }

}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Command
{

    pub id: Option<u32>,
    pub command: String, //Optional when namespaces get added.
    pub type_name: Option<SupportedType>,
    pub params: Option<Vec<Option<TypeInstance>>>
    
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CommandResult
{

    pub id: Option<u32>,
    pub result: Option<TypeInstance>,
    //pub message: Option<SendableText>,
    pub done: bool

}

impl CommandResult
{

    pub fn new(id: Option<u32>, result: Option<TypeInstance>, done: bool) -> Self
    {

        Self
        {

            id,
            result,
            done

        }

    }

    pub fn done(command: &Command, result: Option<TypeInstance>) -> Self
    {

        Self
        {

            id: command.id,
            result,
            done: true

        }

    }
    
    pub fn not_done(command: &Command, result: Option<TypeInstance>) -> Self
    {

        Self
        {

            id: command.id,
            result,
            done: false

        }

    }

}

#[derive(Serialize, Deserialize, Debug, Default)] //Default, 
pub struct CommandError
{

    pub id: Option<u32>,
    pub message: SendableText,
    pub index: Option<usize>,
    pub found_type: Option<SendableText>

}

impl CommandError
{

    pub fn new(command: &Command, message: SendableText) -> Self
    {

        Self
        {

            id: command.id,
            message,
            index: None,
            found_type: None

        }

    }

    pub fn at_index(command: &Command, message: SendableText, index: usize) -> Self //id: Option<u32>, 
    {

        Self
        {

            id: command.id,
            message,
            index:Some(index),
            found_type: None

        }

    }

    pub fn at_index_with_found_type(command: &Command, message: SendableText, index: usize, found_type: SendableText) -> Self
    {

        Self
        {

            id: command.id,
            message,
            index: Some(index),
            found_type: Some(found_type)

        }

    }

    pub fn invalid_command(command: &Command) -> Self
    {

        CommandError::new(command, SendableText::Str("Invalid command provided.")) //command.id,

    }

    pub fn not_implemented(command: &Command) -> Self
    {

        CommandError::new(command, SendableText::Str("Not implemented"))

    }

    pub fn invalid_command_for_the_specified_type(command: &Command) -> Self
    {

        CommandError::new(command, SendableText::Str("Invalid command for the specified type."))

    }

}

impl Display for CommandError
{

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {

        write!(f, "Message: {}, Id: {:#?}", self.message, self.id)       

    }

}

impl Error for CommandError
{

    fn source(&self) -> Option<&(dyn Error + 'static)>
    {

        None

    }

    fn description(&self) -> &str
    {

        "description() is deprecated; use Display"

    }

    fn cause(&self) -> Option<&dyn Error>
    {

        self.source()

    }

    //fn provide<'a>(&'a self, request: &mut std::error::Request<'a>) {}
}


#[derive(Debug, Default, Serialize, Deserialize, EnumString, FromRepr, EnumIter, AsRefStr, EnumCount, IntoStaticStr)]
pub enum StreamedMessage
{

    Command(Command),
    CommandResult(CommandResult),
    CommandError(CommandError),
    Error(SendableText),
    #[default]
    Empty

}