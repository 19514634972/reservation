mod errors;
pub use error::ReservationError;


pub type ReservationId=String;
pub type UserId=String;
pub type ResourceId=String;

pub trait Rsvp{
    fn reserve(&self,rsvp: abi::Reservation)->Result<abi::Reservation,ReservationError>;
    fn change_status(&self,id:ReservationId)->Result<abi::Reservation,ReservationError>;
    fn update_note(&self,id:ReservationId,note: String)->Result<abi::Reservation,ReservationError>;
    fn delete(&self,id:ReservationId)->Result<(),ReservationError>;
    fn get(&self,id:ReservationId)->Result<abi::Reservation,ReservationError>;
    fn query(&self,query:abi::UserId,)->Result<Vect<abi::Reservation>,ReservationError>;


}