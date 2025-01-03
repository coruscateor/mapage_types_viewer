
#[derive(Default, Clone, Copy, Debug)]
pub enum AllOrNot<T>
{

    #[default]
    All,
    NotAll(T)

}

impl<T> AllOrNot<T>
{

    pub fn is_all(&self) -> bool
    {

        match self
        {

            AllOrNot::All => true,
            AllOrNot::NotAll(_) => false

        }

    }
    
    pub fn is_not_all(&self) -> bool
    {

        match self
        {

            
            AllOrNot::All => false,
            AllOrNot::NotAll(_) => true

        }

    }

}