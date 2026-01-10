//Model
pub enum TaskState{
    Pending,
    Completed,
    Dropped,
}

pub enum Action{
    Add(String),
    Mark_As(id: usize, state: TaskState),
    List,
    Quit,
    Invalid,
}

pub struct Task{
   pub title: String,
   pub id: usize,    
   pub state: TaskState,  
}

//Funcitons
pub fn set_state() -> String{
//todo(): translate String to TaskState
} 


impl Task {
// todo(): Write action fn()
}
